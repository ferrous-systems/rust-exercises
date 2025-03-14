# Cpp Interop

This crate also includes a cpp header-only library [rapidcsv](https://github.com/d99kris/rapidcsv) that can help use read and process CSV files like so:

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

