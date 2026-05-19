import 'package:flutter/material.dart';
import 'package:user_interface/screens/helper/welcome.screen.dart';

class AcendlyFrontendClientRootController extends StatefulWidget {
    const AcendlyFrontendClientRootController({super.key});

    @override
    State<AcendlyFrontendClientRootController> createState() => _AcendlyFrontendClientRootControllerState();
}

class _AcendlyFrontendClientRootControllerState extends State<AcendlyFrontendClientRootController> {
    @override
    Widget build(BuildContext context) {
        return WelcomeScreenComponent();
    }
}
