Their are many instances in game development where its important to know the compass facing for a given direction. This is particularly true in 2D games that use four or eight direction sprites. 

In order to make this easier the enums CompassQuadrant (for four way) and CompassOctant (for eight way) have been added with implementations of From<Dir2> for ease of use. 
