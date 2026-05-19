import 'package:user_interface/constants/acendly_root_window.constants.dart';
import 'package:window_manager/window_manager.dart';
import 'package:flutter/material.dart';

class AcendlyFrontendClientRootWindowConfiguration {
  static final AcendlyFrontendClientRootWindowConfiguration current =
      AcendlyFrontendClientRootWindowConfiguration._internal();

  AcendlyFrontendClientRootWindowConfiguration._internal();

  Future<void> initWindowConfiguration() async {
    WidgetsFlutterBinding.ensureInitialized();
    await windowManager.ensureInitialized();
    final screenSize = await windowManager.getSize();

    final windowHeight =
        (screenSize.height *
                AcendlyRootWindowConstants.current.windowScreenHeightRatio)
            .clamp(
              AcendlyRootWindowConstants.current.windowScreenMinHeightLimit,
              double.infinity,
            );
    final windowWidth =
        (screenSize.width *
                AcendlyRootWindowConstants.current.windowScreenWidthRatio)
            .clamp(
              AcendlyRootWindowConstants.current.windowScreenMinWidthLimit,
              double.infinity,
            );

    final windowOption = WindowOptions(
      size: Size(windowWidth, windowHeight),
      minimumSize: Size(
        AcendlyRootWindowConstants.current.windowScreenMinWidthLimit,
        AcendlyRootWindowConstants.current.windowScreenMinHeightLimit,
      ),
      center: true,
      backgroundColor: Colors.transparent,
      skipTaskbar: true,
      titleBarStyle: TitleBarStyle.hidden,
    );

    windowManager.waitUntilReadyToShow(windowOption, () async {
      await windowManager.setAsFrameless();
      await windowManager.show();
      await windowManager.focus();
    });
  }
}
