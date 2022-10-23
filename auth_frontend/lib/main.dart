import 'package:oneauth/pages/login.dart';
import 'package:oneauth/util/lang_controller.dart';
import 'package:oneauth/util/theme_controller.dart';
import 'package:flutter/material.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:oneauth/constants/themes/light_theme.dart' as light;
import 'package:oneauth/constants/themes/dark_theme.dart' as dark;

Future<void> main() async {
  // ensure that we have the scheduler binding initialized
  WidgetsFlutterBinding.ensureInitialized();
  // load the shared preferences from disk before the app is started
  final prefs = await SharedPreferences.getInstance();

  runApp(MyApp(
      themeController: ThemeController(prefs),
      languageController: await LanguageController.createController(prefs)));
}

class MyApp extends StatelessWidget {
  static const String title = 'OneAuth';

  final ThemeController themeController;
  final LanguageController languageController;

  const MyApp(
      {Key? key,
      required this.themeController,
      required this.languageController})
      : super(key: key);

  @override
  Widget build(BuildContext context) {
    // use AnimatedBuilder to listen to theme changes (listen to ChangeNotifier)
    // the app will be rebuilt when the theme changes
    return AnimatedBuilder(
      animation: themeController,
      builder: (context, _) {
        return LanguageControllerProvider(
            controller: languageController,
            child: ThemeControllerProvider(
              controller: themeController,
              child: MaterialApp(
                  title: title,
                  theme: _buildCurrentTheme(),
                  home: const LoginPage()),
            ));
      },
    );
  }

  // build the flutter theme from the saved theme string
  ThemeData _buildCurrentTheme() {
    switch (themeController.currentTheme) {
      case "dark":
        return dark.getTheme();
      case "light":
      default:
        return light.getTheme();
    }
  }
}
