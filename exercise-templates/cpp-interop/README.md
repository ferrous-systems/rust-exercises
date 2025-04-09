# Cpp Interop

This crate depends on a cpp header-only library [rapidcsv](https://github.com/d99kris/rapidcsv) that can read and process CSV files.

You can download it on Linux/MacOS with:

```console
curl -o src/rapidcsv.h https://raw.githubusercontent.com/d99kris/rapidcsv/a98b85e663114b8fdc9c0dc03abf22c296f38241/src/rapidcsv.h
```

and on Windows with Powershell with:

```console
Invoke-WebRequest -URI "https://raw.githubusercontent.com/d99kris/rapidcsv/a98b85e663114b8fdc9c0dc03abf22c296f38241/src/rapidcsv.h" -OutFile src/rapidcsv.h
```

With either method, make sure `rapidcsv.h` is found inside the `cpp-interop/src` folder.

You can try using `rapidcsv.h` on the `example.cpp` file

```
#include <iostream>
#include <vector>
#include "rapidcsv.h"

int main()
{
  rapidcsv::Document doc("example.csv");

  std::vector<float> col = doc.GetColumn<float>("Close");
  std::cout << "Read " << col.size() << " values." << std::endl;
}
```

 by running the command

 ```
 ❯ clang -lstdc++ -std=c++11 -I src src/example.cpp
 # or with GCC
❯ g++ -std=c++11 -I src src/example.cpp
 ```

