import org.jetbrains.compose.desktop.application.dsl.TargetFormat
import org.jetbrains.kotlin.gradle.dsl.JvmTarget

plugins {
    alias(libs.plugins.kotlinMultiplatform)
    alias(libs.plugins.androidApplication)
    alias(libs.plugins.composeMultiplatform)
    alias(libs.plugins.composeCompiler)
    alias(libs.plugins.composeHotReload)
}

kotlin {
    androidTarget {
        compilerOptions {
            jvmTarget.set(JvmTarget.JVM_11)
        }
    }

    // Temporarily disable. As of me writing this, this was disabled because the
    // Material3 Adaptive Navigation3 doesn't support iOS, and I'm not even making it work
    // on iOS as of now, so chuck it in the chuck bucket I guess
    // listOf(
    //     iosArm64(),
    //     iosSimulatorArm64()
    // ).forEach { iosTarget ->
    //     iosTarget.binaries.framework {
    //         baseName = "LANCEmate"
    //         isStatic = true
    //     }
    // }

    jvm()

    sourceSets {
        androidMain.dependencies {
            implementation(libs.compose.uiToolingPreview)
            implementation(libs.androidx.activity.compose)
            implementation(libs.moko.permissions.storage)
            implementation(libs.moko.permissions.motion)
            implementation(libs.moko.permissions.location)
            implementation(libs.moko.permissions.bluetooth)
            api(libs.moko.permissions)
            api(libs.moko.permissions.compose)
        }
        commonMain.dependencies {
            implementation(libs.compose.runtime)
            implementation(libs.compose.foundation)
            implementation(libs.compose.material3)
            implementation(libs.compose.ui)
            implementation(libs.compose.components.resources)
            implementation(libs.compose.uiToolingPreview)
            implementation(libs.androidx.lifecycle.viewmodelCompose)
            implementation(libs.androidx.lifecycle.runtimeCompose)
            implementation(libs.blue.falcon)
            implementation(libs.navigation3.ui)
            implementation(libs.compose.material3.navigation3)
            implementation(libs.compose.material3.adaptive)
            implementation(libs.compose.material3.adaptive.navigationSuite)
            implementation(libs.kotlinx.serialization.core)
            implementation(libs.koalaplot)
        }
        commonTest.dependencies {
            implementation(libs.kotlin.test)
        }
        jvmMain.dependencies {
            implementation(compose.desktop.currentOs)
            implementation(libs.kotlinx.coroutinesSwing)
            implementation(libs.flatlaf)
        }
        iosMain.dependencies {
            // TODO: No idea if I will actually need this guy (I think this is the IMU
            //  permission thing)
            implementation(libs.moko.permissions.motion)
            implementation(libs.moko.permissions.storage)
            implementation(libs.moko.permissions.location)
            implementation(libs.moko.permissions.bluetooth)
            api(libs.moko.permissions)
            api(libs.moko.permissions.compose)
        }
    }
}

android {
    namespace = "com.arvenora.lancemate"
    compileSdk = libs.versions.android.compileSdk.get().toInt()

    defaultConfig {
        applicationId = "com.arvenora.lancemate"
        minSdk = libs.versions.android.minSdk.get().toInt()
        targetSdk = libs.versions.android.targetSdk.get().toInt()
        versionCode = 1
        versionName = "1.0"
    }
    packaging {
        resources {
            excludes += "/META-INF/{AL2.0,LGPL2.1}"
        }
    }
    buildTypes {
        getByName("release") {
            isMinifyEnabled = false
        }
    }
    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_11
        targetCompatibility = JavaVersion.VERSION_11
    }
}

dependencies {
    debugImplementation(libs.compose.uiTooling)
}

compose.desktop {
    application {
        mainClass = "com.arvenora.lancemate.MainKt"

        nativeDistributions {
            targetFormats(TargetFormat.Dmg, TargetFormat.Msi, TargetFormat.Deb)
            packageName = "com.arvenora.lancemate"
            packageVersion = "1.0.0"
        }
    }
}

tasks.matching { it.name == "syncComposeResourcesForIos" }
    .configureEach { enabled = false }

configurations.configureEach {
    resolutionStrategy.eachDependency {
        if (requested.group == "org.jetbrains.skiko") {
            useVersion("0.9.41") // example: pick one version and keep it consistent
            because("Avoid Skiko JVM/native mismatch (MetalApiKt JNI symbols)")
        }
    }
}