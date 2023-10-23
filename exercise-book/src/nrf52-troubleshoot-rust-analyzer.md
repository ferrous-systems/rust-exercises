# ▶ Run button, type annotations and syntax highlighting missing / Rust-Analyzer is not working

If you get no type annotations, no "Run" button and no syntax highlighting this means Rust-Analyzer isn't at work yet.

Try the following:

- add something to the file you're currently looking at, delete it again and save. This triggers a re-run. (you can also `touch` the file in question)
- check that you have a single folder open in VS code; this is different from a single-folder VS code workspace. First close all the currently open folders then open a single folder using the 'File > Open Folder' menu. The open folder should be the `beginner/apps` folder for the beginner workshop or the `advanced/firmware` folder for the advanced workshop.

- use the latest version of the Rust-Analyzer plugin. If you get a prompt to update the Rust-Analyzer extension when you start VS code accept it. You may also get a prompt about updating the Rust-Analayzer binary; accept that one too. The extension should restart automatically after the update. If it doesn't then close and re-open VS code.

- You may need to wait a little while Rust-Analyzer analyzes all the crates in the dependency graph. Then you may need to modify and save the currently open file to force Rust-Analyzer to analyze it.
