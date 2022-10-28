import 'package:day_night_switcher/day_night_switcher.dart';
import 'package:flutter/material.dart';
import 'package:oneauth/util/lang/language.dart';
import 'package:oneauth/util/lang_controller.dart';
import 'package:oneauth/util/theme_controller.dart';
import 'package:wave/config.dart';
import 'package:wave/wave.dart';

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
  bool _obscureText = true;

  void _toggle() {
    setState(() {
      _obscureText = !_obscureText;
    });
  }

  @override
  Widget build(BuildContext context) {
    final lang = LanguageController.of(context);
    final isDarkMode = ThemeController.of(context).currentTheme == 'dark';
    return Scaffold(
      appBar: AppBar(
        title: Text(lang.getTranslation('login-title')),
        actions: [
          SizedBox(
            height: double.infinity,
            child: DayNightSwitcherIcon(
              isDarkModeEnabled: isDarkMode,
              // do nothing on state change
              onStateChanged: (_) {
                ThemeController.of(context).toggleTheme();
              },
            ),
          ),
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
      body: Stack(children: [
        // wave background
        Positioned(
          top: 0,
          left: 0,
          right: 0,
          child: SizedBox(
            // take all the screen
            height: MediaQuery.of(context).size.height,
            width: MediaQuery.of(context).size.width,
            child: WaveWidget(
              config: CustomConfig(
                // take system colors
                colors: [
                  const Color(0xFF114B5F).withOpacity(0.5),
                  const Color(0xFF456990).withOpacity(0.5),
                  const Color(0xFFE4FDE1).withOpacity(0.5),
                ],
                durations: [35000, 19440, 10800],
                heightPercentages: [0.20, 0.23, 0.25],
              ),
              size: const Size(double.infinity, double.infinity),
              waveAmplitude: 0,
            ),
          ),
        ),
        Center(
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: <Widget>[
              // the login box with a form that does not fills the screen
              Container(
                // semi-transparent
                color: Theme.of(context).cardColor.withOpacity(0.5),
                width: 300,
                child: Center(
                  // a card that does not fill the screen
                  // with rounded corners
                  child: Card(
                    shape: RoundedRectangleBorder(
                      borderRadius: BorderRadius.circular(15.0),
                    ),
                    child: Padding(
                      padding: const EdgeInsets.all(16.0),
                      child: Column(
                        mainAxisSize: MainAxisSize.min,
                        children: <Widget>[
                          // the title
                          Text(
                            lang.getTranslation('login-title'),
                            style: Theme.of(context).textTheme.headline6,
                          ),
                          // the form
                          Form(
                            child: Column(
                              children: <Widget>[
                                // the email field
                                TextFormField(
                                  decoration: InputDecoration(
                                    labelText: lang
                                        .getTranslation('username-or-email'),
                                  ),
                                ),
                                // the password field (with a button to show/hide the password)
                                TextFormField(
                                  decoration: InputDecoration(
                                    labelText: lang.getTranslation('password'),
                                    suffixIcon: IconButton(
                                      icon: const Icon(Icons.remove_red_eye),
                                      onPressed: () {
                                        _toggle();
                                      },
                                    ),
                                  ),
                                  obscureText: _obscureText,
                                ),
                                // forgot password text (right aligned)
                                Align(
                                  alignment: Alignment.centerRight,
                                  child: TextButton(
                                    onPressed: () {},
                                    child: Text(
                                      lang.getTranslation('forgot-password'),
                                      style: Theme.of(context)
                                          .textTheme
                                          .bodyText2!
                                          .copyWith(
                                            color: Theme.of(context)
                                                .colorScheme
                                                .secondary,
                                          ),
                                    ),
                                  ),
                                ),
                                // the login button
                                Padding(
                                  padding: const EdgeInsets.symmetric(
                                      vertical: 16.0),
                                  child: ElevatedButton(
                                    onPressed: () {
                                      // do nothing
                                    },
                                    child: Text(lang.getTranslation('login')),
                                  ),
                                ),
                              ],
                            ),
                          ),
                        ],
                      ),
                    ),
                  ),
                ),
              ),
            ],
          ),
        ),
      ]),
    );
  }
}
