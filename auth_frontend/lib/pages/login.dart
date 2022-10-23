import 'package:flutter/material.dart';
import 'package:oneauth/util/lang_controller.dart';

/// Example app widget
class LoginPage extends StatelessWidget {
  /// Main app widget.
  const LoginPage({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return const LoginScreen();
  }
}

/// Example login screen
class LoginScreen extends StatefulWidget {
  const LoginScreen({Key? key}) : super(key: key);

  @override
  State<LoginScreen> createState() => _LoginScreenState();
}

class _LoginScreenState extends State<LoginScreen> {
  @override
  Widget build(BuildContext context) {
    final lang = LanguageController.of(context);
    return Scaffold(
      appBar: AppBar(
        title: Text(lang.getTranslation('login-title')),
      ),
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: <Widget>[
            Text(
              lang.getTranslation("current-lang-name"),
            ),
          ],
        ),
      ),
    );
  }
}
