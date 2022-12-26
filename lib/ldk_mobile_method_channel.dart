import 'package:flutter/foundation.dart';
import 'package:flutter/services.dart';

import 'ldk_mobile_platform_interface.dart';

/// An implementation of [LdkMobilePlatform] that uses method channels.
class MethodChannelLdkMobile extends LdkMobilePlatform {
  /// The method channel used to interact with the native platform.
  @visibleForTesting
  final methodChannel = const MethodChannel('ldk_mobile');

  @override
  Future<String?> getPlatformVersion() async {
    final version = await methodChannel.invokeMethod<String>('getPlatformVersion');
    return version;
  }
}
