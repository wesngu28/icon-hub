# Icon Hub

A Tauri project that collects the icons on your desktop and taskbar and aims to provide a central GUI to customize them for those who care about theming consistency on Windows devices.

On hold since 2/16, didn't initialize git until 3/29

General idea:
1. Render taskbar and desktop icons to react app.
2. Assign onclick handlers to each taskbar item which will change the active item in the gui. Clicking some change button will invoke rust fn in the backend that will take in a photo input to change the icons