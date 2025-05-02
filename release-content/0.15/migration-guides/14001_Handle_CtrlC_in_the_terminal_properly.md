If you are overriding the `Ctrl+C` handler then you should call `TerminalCtrlCHandlerPlugin::gracefully_exit` from your handler. It will tell the app to exit.
