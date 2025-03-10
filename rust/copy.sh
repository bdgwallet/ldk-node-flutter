       if [ -d "../android/src/main/jniLibs" ]; then rm -r ../android/src/main/jniLibs
        fi
        if [ -e "../ios/librust_ldk_node.a" ]; then rm ../ios/librust_ldk_node.a
        fi

       mkdir -p ../android/src/main/jniLibs/arm64-v8a
       mkdir -p ../android/src/main/jniLibs/armeabi-v7a
       mkdir -p ../android/src/main/jniLibs/x86

       cp target/aarch64-linux-android/release/librust_ldk_node.so  ../android/src/main/jniLibs/arm64-v8a
       cp target/armv7-linux-androideabi/release/librust_ldk_node.so  ../android/src/main/jniLibs/armeabi-v7a
       cp target/i686-linux-android/release/librust_ldk_node.so  ../android/src/main/jniLibs/x86

       cp target/universal/release/librust_ldk_node.a  ../ios/

