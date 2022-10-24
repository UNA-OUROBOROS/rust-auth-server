import 'package:flutter/material.dart';

class Language {
  final String _code;
  final String _name;
  final Image _flag;

  Language(this._code, this._name, this._flag);

  String get code => _code;
  String get name => _name;
  Image get flag => _flag;

  static Future<Language> fromJson(dynamic languageJson) async {
    if (languageJson is Map<String, dynamic>) {
      if (languageJson.containsKey('code') &&
          languageJson.containsKey('name')) {
        String code = languageJson['code'];
        String name = languageJson['name'];
        Image flag = Image.asset('assets/lang/flags/$code.png');
        return Language(code, name, flag);
      }
      throw LanguageDeserializeException(languageJson);
    }
    throw Exception("languageJson is not a json object");
  }
}

class LanguageDeserializeException implements Exception {
  LanguageDeserializeException(this.languageJson);

  final Map<String, dynamic> languageJson;

  @override
  String toString() {
    return "error deserialing language from json";
  }
}
