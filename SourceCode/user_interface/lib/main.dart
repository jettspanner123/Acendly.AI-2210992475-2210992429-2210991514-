import 'package:flutter/material.dart';
import 'package:user_interface/configurations/acendly_root_window.configuration.dart';
import 'package:user_interface/controllers/acendly_frontend_client_root.controller.dart';

void main() async {
    await AcendlyFrontendClientRootWindowConfiguration.current.initWindowConfiguration();
    runApp(const MyApp());
}

class MyApp extends StatelessWidget {
    const MyApp({super.key});

    @override
    Widget build(BuildContext context) {
        return MaterialApp(
            title: 'Flutter Demo',
            theme: ThemeData(colorScheme: .fromSeed(seedColor: Colors.deepPurple)),
            home: AcendlyFrontendClientRootController(),
        );
    }
}
