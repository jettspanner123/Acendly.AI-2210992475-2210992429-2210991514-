import 'package:flutter/material.dart';

class AcendlyRootWindowConstants {
  static final AcendlyRootWindowConstants current =
      AcendlyRootWindowConstants._internal();

  AcendlyRootWindowConstants._internal();

  final double windowScreenWidthRatio = 0.8;
  final double windowScreenHeightRatio = 0.8;

  final double windowScreenMinWidthLimit = 800;
  final double windowScreenMinHeightLimit = 1000;
}
