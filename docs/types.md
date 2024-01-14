# Rage Types

## Basic Types
```rage 
blob:<SIZE>
b8, b16, b32, b64, b128, ...
```

## Data Types

### Structs

```rage 
Color {
    red     u8,
    green   u8,
    blue    u8,
}

color = Color { 0, 255, 0 }
```

### Union
```rage 
Value {
    | bool
    | u8
}
v1 = Value { true } 
v2 = Value { 1 }
assert(v1 == v2)
```

### Enum / Tagged Union
```rage 
MyColor {
    | Red 
    | Green 
    | Blue 
    | Custom {
        r u8,
        g u8,
        b u8,
    }
}
red = MyColor:Red
custom = MyColor:Custom { 127, 0, 255 }
```
