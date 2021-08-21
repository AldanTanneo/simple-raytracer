pub mod morton_code;

use crate::hittable::hittable_list::HittableList;
use crate::hittable::{Hit, Hittable};
use crate::ray::Ray;
use crate::vec3::Point3;
use crate::FastRng;

use morton_code::{find_split, morton_code};

#[derive(Clone, Debug)]
pub struct BoundingBox {
    pub minimum: Point3,
    pub maximum: Point3,
}

impl BoundingBox {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool {
        let inv_direction = Point3::ONES / ray.direction;
        let mut t0 = (self.minimum - ray.origin) * inv_direction;
        let mut t1 = (self.maximum - ray.origin) * inv_direction;
        if inv_direction.x < 0.0 {
            std::mem::swap(&mut t0.x, &mut t1.x);
        }
        if inv_direction.y < 0.0 {
            std::mem::swap(&mut t0.y, &mut t1.y);
        }
        if inv_direction.z < 0.0 {
            std::mem::swap(&mut t0.z, &mut t1.z);
        }

        let t_min = t0.max(Point3::ONES * t_min);
        let t_max = t1.min(Point3::ONES * t_max);

        t_max.x > t_min.x && t_max.y > t_min.y && t_max.z > t_min.z
    }

    pub fn join(&self, other: &Self) -> Self {
        Self {
            minimum: self.minimum.min(other.minimum),
            maximum: self.maximum.max(other.maximum),
        }
    }

    fn center(&self) -> Point3 {
        (self.minimum + self.maximum) * 0.5
    }

    /* fn area(&self) -> f64 {
        let tmp = self.maximum - self.minimum;
        2.0 * (tmp.x * tmp.y + tmp.y * tmp.z + tmp.z * tmp.x)
    } */
}

#[derive(Debug)]
enum SubHierarchy<'a> {
    Object(&'a Box<dyn Hittable + 'a>),
    SubHierarchies {
        left: Box<BoundingVolumeHierarchy<'a>>,
        right: Box<BoundingVolumeHierarchy<'a>>,
    },
}

#[derive(Debug)]
pub struct BoundingVolumeHierarchy<'a> {
    bounding_box: BoundingBox,
    sub_hierarchy: SubHierarchy<'a>,
}

use SubHierarchy::*;

impl<'a> BoundingVolumeHierarchy<'a> {
    pub fn build(world: &'a HittableList<'a>) -> anyhow::Result<Self> {
        if world.len() == 0 {
            Err(anyhow::anyhow!("Could not create BVH from empty scene."))
        } else {
            let mut max = f64::NEG_INFINITY;
            let mut min: f64 = f64::INFINITY;
            let mut precomputed_bounding_boxes: Vec<_> = world
                .iter()
                .map(|obj| {
                    let bbox = obj.bounding_box();
                    let center = bbox.center();
                    let (min_c, max_c) = center.min_max_coords();
                    if min_c < min {
                        min = min_c - 1e-10;
                    }
                    if max_c > max {
                        max = max_c + 1e-10;
                    }
                    (obj, bbox, center)
                })
                .collect::<Vec<_>>()
                .into_iter()
                .map(|(obj, bbox, center)| (obj, bbox, morton_code((center - min) / (max - min))))
                .collect();
            precomputed_bounding_boxes.sort_unstable_by_key(|elt| elt.2);
            Ok(Self::from_list(&precomputed_bounding_boxes[..]))
        }
    }

    fn from_list(list: &[(&'a Box<dyn Hittable + 'a>, BoundingBox, u128)]) -> Self {
        match list {
            [] => unreachable!(),
            [(obj, bbox, _)] => Self {
                bounding_box: bbox.clone(),
                sub_hierarchy: Object(*obj),
            },
            [(a, box_a, _), (b, box_b, _)] => {
                let left_tree = Self {
                    bounding_box: box_a.clone(),
                    sub_hierarchy: Object(*a),
                };
                let right_tree = Self {
                    bounding_box: box_b.clone(),
                    sub_hierarchy: Object(*b),
                };

                BoundingVolumeHierarchy {
                    bounding_box: box_a.join(box_b),
                    sub_hierarchy: SubHierarchies {
                        left: Box::new(left_tree),
                        right: Box::new(right_tree),
                    },
                }
            }
            list => {
                let mut middle_point = find_split(list);
                if middle_point > list.len() - 1 || middle_point == 0 {
                    println!("uh oh: 0..{} gave {}", list.len(), middle_point);
                    if let [(_, _, a), (_, _, b), (_, _, c)] = list {
                        println!("    {:b}\n    {:b}\n    {:b}", a, b, c);
                    }
                    middle_point = list.len() / 2;
                }
                let left_tree = Self::from_list(&list[0..middle_point]);
                let right_tree = Self::from_list(&list[middle_point..]);
                Self {
                    bounding_box: left_tree.bounding_box.join(&right_tree.bounding_box),
                    sub_hierarchy: SubHierarchies {
                        left: Box::new(left_tree),
                        right: Box::new(right_tree),
                    },
                }
            }
        }
    }

    pub fn depth_and_num_nodes(&self) -> (usize, usize) {
        match &self.sub_hierarchy {
            Object(_) => (1, 1),
            SubHierarchies { left, right } => {
                let (depth_left, nodes_left) = left.depth_and_num_nodes();
                let (depth_right, nodes_right) = right.depth_and_num_nodes();
                (1 + depth_left.max(depth_right), nodes_left + nodes_right)
            }
        }
    }
}

impl<'a> Hittable for BoundingVolumeHierarchy<'a> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rng: &mut FastRng) -> Hit {
        if self.bounding_box.hit(ray, t_min, t_max) {
            match &self.sub_hierarchy {
                Object(object) => object.hit(ray, t_min, t_max, rng),
                SubHierarchies { left, right, .. } => {
                    if let Some(left_record) = left.hit(ray, t_min, t_max, rng) {
                        if let Some(right_record) = right.hit(ray, t_min, left_record.time, rng) {
                            Some(right_record)
                        } else {
                            Some(left_record)
                        }
                    } else {
                        right.hit(ray, t_min, t_max, rng)
                    }
                }
            }
        } else {
            None
        }
    }

    fn bounding_box(&self) -> BoundingBox {
        self.bounding_box.clone()
    }
}

impl<'a> std::fmt::Display for BoundingVolumeHierarchy<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.sub_hierarchy {
            Object(_) => {
                write!(f, "Object{:?}", self.bounding_box.center().components())
            }
            SubHierarchies { left, right } => {
                write!(f, "SubHierarchies(left: {}, right: {})", left, right)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::vec3::Vec3;

    #[test]
    fn test_swap_from_structs() {
        let mut a = Vec3::X + Vec3::Y;
        let mut b = Vec3::Z;

        std::mem::swap(&mut a.z, &mut b.z);

        assert_eq!((a, b), (Vec3::ONES, Vec3::ZEROS));
    }
}
