import 'package:flutter/material.dart';

ThemeData getTheme() {
  ThemeData themeData = ThemeData.from(colorScheme: const ColorScheme.dark());

  return themeData.copyWith(
    appBarTheme: const AppBarTheme(backgroundColor: Color(0xFF282828)),
  );
}
