import 'package:flutter/material.dart';
import 'package:flutter/scheduler.dart';
import 'package:shared_preferences/shared_preferences.dart';

/// provides the currently selected theme, saves changed theme preferences to disk
class ThemeController extends ChangeNotifier {
  static const themePrefKey = 'theme';
  static const preferSystemColorPrefKey = 'preferOSColorScheme';

  ThemeController(this._prefs) {
    bool preferSystemColor = _prefs.getBool(preferSystemColorPrefKey) ?? true;
    String? theme = _prefs.getString(themePrefKey);
    _currentTheme = theme ?? 'light';
    // override the theme if the user prefers the System color scheme
    if (preferSystemColor) {
      var brightness = SchedulerBinding.instance.window.platformBrightness;
      _currentTheme = brightness == Brightness.dark ? 'dark' : 'light';
    }
  }

  void toggleTheme() {
    setTheme(_currentTheme == 'light' ? 'dark' : 'light');
  }

  final SharedPreferences _prefs;
  late String _currentTheme;
  late bool _preferSystemColorScheme;

  /// get the current theme
  String get currentTheme => _currentTheme;

  /// get if prefer system color scheme is enabled
  bool get preferSystemColorScheme => _preferSystemColorScheme;

  void setTheme(String theme) {
    _currentTheme = theme;

    // notify the app that the theme was changed
    notifyListeners();

    // store updated theme on disk
    _prefs.setString(themePrefKey, theme);
  }

  void setPreferSystemColorScheme(bool preferSystemColorScheme) {
    _preferSystemColorScheme = preferSystemColorScheme;

    // notify the app that the theme was changed
    notifyListeners();

    // store updated preferSystemColorScheme on disk
    _prefs.setBool(preferSystemColorPrefKey, preferSystemColorScheme);
  }

  /// get the controller from any page of your app
  static ThemeController of(BuildContext context) {
    final provider =
        context.dependOnInheritedWidgetOfExactType<ThemeControllerProvider>()
            as ThemeControllerProvider;
    return provider.controller;
  }
}

/// provides the theme controller to any page of your app
class ThemeControllerProvider extends InheritedWidget {
  const ThemeControllerProvider({
    Key? key,
    required this.controller,
    required Widget child,
  }) : super(key: key, child: child);

  final ThemeController controller;

  @override
  bool updateShouldNotify(ThemeControllerProvider oldWidget) =>
      controller != oldWidget.controller;
}
