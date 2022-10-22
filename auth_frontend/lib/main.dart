import 'package:oneauth/pages/login.dart';
import 'package:oneauth/style/theme_controller.dart';
import 'package:flutter/material.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:oneauth/constants/themes/light_theme.dart' as light;
import 'package:oneauth/constants/themes/dark_theme.dart' as dark;

Future<void> main() async {
  // ensure that we have the scheduler binding initialized
  WidgetsFlutterBinding.ensureInitialized();
  // load the shared preferences from disk before the app is started
  final prefs = await SharedPreferences.getInstance();

  // create new theme controller, which will get the currently selected from shared preferences
  final themeController = ThemeController(prefs);
  runApp(MyApp(themeController: themeController));
}

class MyApp extends StatelessWidget {
  static const String title = 'OneAuth';

  final ThemeController themeController;

  const MyApp({Key? key, required this.themeController}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    // use AnimatedBuilder to listen to theme changes (listen to ChangeNotifier)
    // the app will be rebuilt when the theme changes
    return AnimatedBuilder(
      animation: themeController,
      builder: (context, _) {
        // wrap app in inherited widget to provide the ThemeController to all pages
        return ThemeControllerProvider(
          controller: themeController,
          child: MaterialApp(
              title: title,
              theme: _buildCurrentTheme(),
              home: const LoginPage(title: title)),
        );
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
