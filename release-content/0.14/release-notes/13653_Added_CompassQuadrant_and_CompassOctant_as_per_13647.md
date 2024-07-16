<!-- Added CompassQuadrant and CompassOctant as per #13647 -->
<!-- https://github.com/bevyengine/bevy/pull/13653 -->

There are many instances in game development where its important to know the compass facing for a given direction. This is particularly true in 2D games that use four or eight direction sprites, or want to map analog input into discrete movement directions.

In order to make this easier the enums `CompassQuadrant` (for a four-way division) and `CompassOctant` (for an eight-way division) have been added with implementations to and `From<Dir2>` for ease of use.
