
extern crate gcc;

fn main() {
	gcc::Config::new()
		.include("/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX10.9.sdk/System/Library/Frameworks/JavaVM.framework/Versions/A/Headers")
		.file("src/interface.c")
		.compile("libinterface.a");
}
