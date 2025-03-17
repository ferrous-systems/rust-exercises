# Cpp Interop

This crate depends on a cpp header-only library [rapidcsv](https://github.com/d99kris/rapidcsv) that can help use read and process CSV files.

You can download it on Linux/MacOS with:

```console
curl -o src/rapidcsv.h https://raw.githubusercontent.com/d99kris/rapidcsv/a98b85e663114b8fdc9c0dc03abf22c296f38241/src/rapidcsv.h
```

and on Windows with Powershell with:

```console
Invoke-WebRequest -URI "https://raw.githubusercontent.com/d99kris/rapidcsv/a98b85e663114b8fdc9c0dc03abf22c296f38241/src/rapidcsv.h"
```

You can try it on the `example.cpp` file:

```
#include <iostream>
#include <vector>
#include "rapidcsv.h"

int main()
{
  rapidcsv::Document doc("examples/colhdr.csv");

  std::vector<float> col = doc.GetColumn<float>("Close");
  std::cout << "Read " << col.size() << " values." << std::endl;
}
```
 Run the example file with from the root of `cpp-interop` via

 ```
 ❯ clang -lstdc++ -std=c++11 -I src src/example.cpp
 # or with GCC
❯ g++ -std=c++11 -I src src/example.cpp
 ```

 The `weather.csv` was taken from [this repository](https://github.com/velicki/Weather_Data_Analysis_Project/blob/main/Weather_Data.csv)

