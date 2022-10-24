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

class TranslationsMetadataNotFoundException implements Exception {
  TranslationsMetadataNotFoundException();

  @override
  String toString() {
    return 'The translations metadata file was not found';
  }
}

class TranslationsMetadataCorruptedException implements Exception {
  TranslationsMetadataCorruptedException(this.cause);

  final Exception? cause;

  @override
  String toString() {
    return 'The translations metadata file is corrupted';
  }
}
