# matrix calculator


## output format (csv)
```csv
{minute past},
t0,
abc,
origin, {x}, {y}, {z}
normal, {x}, {y}, {z}
def,
origin, {x}, {y}, {z}
normal, {x}, {y}, {z}
t1,
abc,
origin, {x}, {y}, {z}
normal, {x}, {y}, {z}
def,
origin, {x}, {y}, {z}
normal, {x}, {y}, {z}
delta,
distance, {v}
rotation,
abc,
eular, {z}, {y}, {z}
ypr, {yaw}, {pitch}, {roll}
def,
eular, {z}, {y}, {z}
ypr, {yaw}, {pitch}, {roll}
.
. /* repeats */
.
```

### remarks on data
- all angles are in radians
- normals are defined as the order a -> b -> c (or d -> e -> f)
- **distance** is the difference between t0's abc-def and t1's abc-def
- rotation is shown in two ways (you should only use one, that makes more sense to you)
    - [eular](https://en.wikipedia.org/wiki/Euler_angles) **in the order of z -> y -> z**
    - [yaw pitch roll](https://en.wikipedia.org/wiki/Aircraft_principal_axes) **z -> y -> x**

## run in linux(ubuntu)
```
./matrix_calc < in.csv > out.csv
```

## building
you will need geometry cradle
```
[rust]
    [geometry]
    [matrix_calc]
```