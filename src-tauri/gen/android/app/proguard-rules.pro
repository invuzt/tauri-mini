# Buang semua yang tidak dipakai
-dontobfuscate
-optimizations !code/simplification/arithmetic,!field/*,!class/merging/*
-keep public class * extends android.app.Activity
-keep public class * extends android.app.Application
-keep class com.tauri.** { *; }
# Hapus log debug
-assumenosideeffects class android.util.Log {
    public static *** d(...);
    public static *** v(...);
}
