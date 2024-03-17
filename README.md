# GUID RS
---
Grid is a demonstration program for driving bare metal graphics without a traditional desktop environment.
Included DRM/KMS (kernel mode setting), GBM (graphics buffer manager), and EGL for rendering content.

Library dependencies:
1. **libc**: involved c language data format (like uint_32t c_void etc.) for an easier life when using FFI play with C libraries.
2. **bitflags**: enhance rust to support bit operation on structured data, very similar to the Flag enum in C#

---
##### Tested on raspberry pi (Raspbian lite) with Waveshare AMOLED screen
![image_01](https://github.com/XionWin/kms/blob/main/resources/image_01.jpg?raw=true)
![image_02](https://github.com/XionWin/kms/blob/main/resources/image_02.jpg?raw=true)

Resources usage is relatively low, and the C# version needs more 50-75% memory (load framework libs) than Rust; the CPU usage is up to 25% higher than the rust version. The result makes sense because the dynamic library executes the core business, and C# and Rust just played as the FFI caller, C# does is a doubling language.
![image_03](https://github.com/XionWin/kms/blob/main/resources/image_03.jpg?raw=true)

---
