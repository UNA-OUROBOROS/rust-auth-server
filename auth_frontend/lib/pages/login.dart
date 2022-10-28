import 'package:flutter/material.dart';
import 'package:oneauth/util/lang/language.dart';
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
        actions: [
          SizedBox(
            height: double.infinity,
            child: ElevatedButton(
              style: ElevatedButton.styleFrom(
                backgroundColor: Colors.transparent,
                elevation: 0,
              ),
              onPressed: () async {
                LanguageController lc = LanguageController.of(context);
                List<Language> languages = await lc.languages;

                // show a dialog to select the language
                showDialog(
                  context: context,
                  builder: (context) => AlertDialog(
                    title: Text(lang.getTranslation('select-a-language')),
                    content: Column(
                      mainAxisSize: MainAxisSize.min,
                      // map its flag and name
                      // also add a default language
                      children:
                          // default system language
                          [
                                Padding(
                                  padding: const EdgeInsets.all(8.0),
                                  child: ListTile(
                                    title: Text(lc
                                        .getTranslation("system-default-lang")),
                                    onTap: () {
                                      lc
                                          .setPreferSystemLanguage(true)
                                          .then((_) {
                                        Navigator.pop(context);
                                        setState(() {});
                                      });
                                    },
                                  ),
                                )
                              ] +
                              languages
                                  .map(
                                    (e) => Padding(
                                      padding: const EdgeInsets.all(8.0),
                                      child: ListTile(
                                        leading: e.flag,
                                        title: Text(e.name),
                                        onTap: () {
                                          lc.setLanguage(e.code).then((c) => c
                                              .setPreferSystemLanguage(false)
                                              .then((_) => {
                                                    Navigator.pop(context),
                                                    setState(() {})
                                                  }));
                                        },
                                      ),
                                    ),
                                  )
                                  .toList(),
                    ),
                  ),
                );
              },
              child: FutureBuilder(
                future: lang.getFlag,
                builder: (context, snapshot) {
                  if (snapshot.hasData) {
                    return snapshot.data as Widget;
                  }
                  return const CircularProgressIndicator();
                },
              ),
            ),
          ),
        ],
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
