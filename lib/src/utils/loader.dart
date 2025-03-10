import 'dart:ffi';
import 'dart:io';

import '../generated/bindings.dart';

final DynamicLibrary dylib = Platform.isIOS
    ? DynamicLibrary.process()
    : Platform.isMacOS
        ? DynamicLibrary.executable()
        : DynamicLibrary.open('librust_ldk_node.so');
RustLdkNodeImpl loaderApi = RustLdkNodeImpl(dylib);
