The gizmos methods function signature changes as follows:

- 2D
  - if it took `position` & `rotation_angle` before -> `Isometry2d::new(position, Rot2::radians(rotation_angle))`
  - if it just took `position` before -> `Isometry2d::from_translation(position)`

- 3D
  - if it took `position` & `rotation` before -> `Isometry3d::new(position, rotation)`
  - if it just took `position` before -> `Isometry3d::from_translation(position)`
