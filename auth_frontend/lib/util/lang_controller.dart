import 'dart:convert';
import 'dart:io';

import 'package:fluent/fluent.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:oneauth/util/lang/exceptions.dart';
import 'package:shared_preferences/shared_preferences.dart';

import 'lang/language.dart';

/// provides information about the current language and its translations
class LanguageController extends ChangeNotifier {
  static const languagePrefKey = 'language';
  static const preferSystemLanguagePrefKey = 'preferSystemLanguage';
  // consists of the base language and the country code(if available)
  // if the language is not available, the default language is used
  // if the the country code is not available take the default language
  // (before the underscore)
  static const _defaultLanguage = 'en_US';

  LanguageController._(this._prefs, String defaultLanguage) {
    String? language = _prefs.getString(languagePrefKey);
    _preferSystemLanguage = _prefs.getBool(preferSystemLanguagePrefKey) ?? true;
    _currentLanguage = language ?? defaultLanguage;
    _bundle = FluentBundle("");
    // override the language if the user prefers the System language
    if (_preferSystemLanguage) {
      _currentLanguage = Platform.localeName;
    }
  }

  static Future<LanguageController> createController(SharedPreferences prefs,
      {String defaultLanguage = LanguageController._defaultLanguage}) async {
    LanguageController controller =
        LanguageController._(prefs, defaultLanguage);
    // load the current language
    await controller.setLanguage(controller._currentLanguage,
        fallback: _defaultLanguage);
    return controller;
  }

  String get currentLanguage => _currentLanguage;
  bool get preferSystemLanguage => _preferSystemLanguage;

  Future<Image> get getFlag async {
    return Image.asset('assets/lang/flags/$_currentLanguage.png');
  }

  // returns a dictionary with all the available languages
  Future<List<Language>> get languages async {
    List<Language> languages = [];
    String infoJson;
    try {
      // get info.json
      infoJson = await rootBundle.loadString('assets/lang/info.json');
    } catch (e) {
      throw TranslationsMetadataNotFoundException();
    }
    // parse the json
    try {
      Map<String, dynamic> info = jsonDecode(infoJson);
      // get the languages
      if (info.containsKey('languages')) {
        List<dynamic> languagesJson = info['languages'];
        for (var languageJson in languagesJson) {
          Language language = await Language.fromJson(languageJson);
          languages.add(language);
        }
      } else {
        throw Exception("languages key not found");
      }
    } on Exception catch (e) {
      throw TranslationsMetadataCorruptedException(e);
    }
    return languages;
  }

  Future<String?> getFallbackLanguage(String language) async {
    String infoJson;
    try {
      // get info.json
      infoJson = await rootBundle.loadString('assets/lang/info.json');
    } catch (e) {
      throw TranslationsMetadataNotFoundException();
    }
    // parse the json
    try {
      Map<String, dynamic> info = jsonDecode(infoJson);
      // get the languages
      if (info.containsKey('fallbacks')) {
        List<dynamic> languagesJson = info['fallbacks'];
        for (var languageJson in languagesJson) {
          String code = languageJson['code'];
          if (code == language) {
            return languageJson['fallback'];
          }
        }
        return null;
      } else {
        throw Exception("languages key not found");
      }
    } on Exception catch (e) {
      throw TranslationsMetadataCorruptedException(e);
    }
  }

  Future<LanguageController> setLanguage(
    String language, {
    // allows to try with the version without the country code
    bool allowGeneric = true,
    // fallback if the requested language is not available
    String? fallback,
  }) async {
    late FluentBundle bundle;
    try {
      bundle = await _loadBundle(language);
    } catch (e) {
      try {
        if (allowGeneric) {
          String? fallbackLanguage = await getFallbackLanguage(language);
          if (fallbackLanguage != null) {
            bundle = await _loadBundle(fallbackLanguage);
          } else {
            String baseLang = language.split('_')[0];
            fallbackLanguage = await getFallbackLanguage(baseLang);
            bundle = await _loadBundle(fallbackLanguage ?? baseLang);
          }
        } else {
          rethrow;
        }
      } catch (e) {
        // try to load the fallback language
        if (fallback != null) {
          bundle = await _loadBundle(fallback);
        }
        // just rethrow the exception if no fallback was provided
        else {
          rethrow;
        }
      }
    }
    _bundle = bundle;
    _currentLanguage = bundle.locale;
    await _prefs.setString(languagePrefKey, language);
    notifyListeners();
    return this;
  }

  Future<LanguageController> setPreferSystemLanguage(
      bool preferSystemLanguage) async {
    // lets check if the system language is available by setting it
    // with fallback to the default language
    try {
      if (preferSystemLanguage) {
        await setLanguage(
          Platform.localeName,
          allowGeneric: true,
          fallback: _defaultLanguage,
        );
      }
      _preferSystemLanguage = preferSystemLanguage;
      _prefs.setBool(preferSystemLanguagePrefKey, preferSystemLanguage);
      notifyListeners();
      return this;
    } catch (e) {
      // we cannot use even the default language, rethrow
      rethrow;
    }
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
      final messages =
          await rootBundle.loadString('assets/lang/translation/$language.ftl');
      final bundle = FluentBundle(language);
      bundle.addMessages(messages);
      return bundle;
    } catch (e) {
      throw TranslationBundleNotFoundException(language);
    }
  }

  late String _currentLanguage;
  late bool _preferSystemLanguage;

  late FluentBundle _bundle;

  final SharedPreferences _prefs;

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
