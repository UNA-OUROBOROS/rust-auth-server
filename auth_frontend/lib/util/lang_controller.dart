import 'package:fluent/fluent.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:shared_preferences/shared_preferences.dart';

/// provides information about the current language and its translations
class LanguageController extends ChangeNotifier {
  static const languagePrefKey = 'language';
  // consists of the base language and the country code(if available)
  // if the language is not available, the default language is used
  // if the the country code is not available take the default language
  // (before the underscore)
  static const _defaultLanguage = 'en_US';

  LanguageController._(
      this._prefs, FluentBundle initialBundle, String defaultLanguage) {
    String? language = _prefs.getString(languagePrefKey);
    _currentLanguage = language ?? defaultLanguage;
    _bundle = initialBundle;
  }

  static Future<LanguageController> createController(SharedPreferences prefs,
      {String defaultLanguage = LanguageController._defaultLanguage}) async {
    FluentBundle bundle = await LanguageController._loadBundle(defaultLanguage);
    return LanguageController._(prefs, bundle, defaultLanguage);
  }

  String get currentLanguage => _currentLanguage;

  void setLanguage(String language) async {
    FluentBundle bundle = await _loadBundle(_currentLanguage);
    _bundle = bundle;
    _currentLanguage = language;
    _prefs.setString(languagePrefKey, language);
    notifyListeners();
  }

  // gets the translation for the given key
  String getTranslation(String message,
      {Map<String, dynamic> args = const {}}) {
    if (_bundle.hasMessage(message)) {
      List<Error> errors = [];
      final translation = _bundle.format(message, args: args, errors: errors);
      if (translation != null) {
        return translation;
      }
      // throw an exception along with the errors
      throw TranslationException(message, errors);
    }
    throw TranslationMessageNotFoundException(message);
  }

  static Future<FluentBundle> _loadBundle(String language) async {
    try {
      final messages = await rootBundle.loadString('assets/lang/$language.ftl');
      final bundle = FluentBundle(language);
      bundle.addMessages(messages);
      return bundle;
    } catch (e) {
      throw TranslationBundleNotFoundException(language);
    }
  }

  final SharedPreferences _prefs;
  late String _currentLanguage;
  late FluentBundle _bundle;

  /// get the controller from any page of your app
  static LanguageController of(BuildContext context) {
    final provider =
        context.dependOnInheritedWidgetOfExactType<LanguageControllerProvider>()
            as LanguageControllerProvider;
    return provider.controller;
  }
}

class LanguageControllerProvider extends InheritedWidget {
  final LanguageController controller;

  const LanguageControllerProvider({
    Key? key,
    required Widget child,
    required this.controller,
  }) : super(key: key, child: child);

  @override
  bool updateShouldNotify(LanguageControllerProvider oldWidget) {
    return controller != oldWidget.controller;
  }
}

class TranslationException implements Exception {
  TranslationException(this.message, this.errors);

  final String message;
  final List<Error> errors;

  @override
  String toString() {
    return '$message: $errors';
  }
}

class TranslationMessageNotFoundException implements Exception {
  TranslationMessageNotFoundException(this.message);

  final String message;

  @override
  String toString() {
    return '$message not found';
  }
}

class TranslationBundleNotFoundException implements Exception {
  TranslationBundleNotFoundException(this.language);

  final String language;

  @override
  String toString() {
    return 'No bundle available for $language';
  }
}
