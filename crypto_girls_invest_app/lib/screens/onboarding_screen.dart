import 'package:flutter/material.dart';
import '../services/privy_service.dart';

class OnboardingScreen extends StatelessWidget {
  final PrivyService _privy = PrivyService();

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: Text('Welcome to CGI')),
      body: Center(
        child: ElevatedButton(
          child: Text('Connect with Privy'),
          onPressed: () async {
            await _privy.connectWallet();
            Navigator.pushNamed(context, '/home');
          },
        ),
      ),
    );
  }
}
