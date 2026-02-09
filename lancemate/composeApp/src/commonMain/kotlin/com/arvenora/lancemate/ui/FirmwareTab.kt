package com.arvenora.lancemate.ui

import androidx.compose.foundation.gestures.Orientation
import androidx.compose.foundation.gestures.scrollable
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.rememberScrollState
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.snapshots.SnapshotStateList
import androidx.compose.ui.Modifier

@Composable
fun FirmwareTab(backStack: SnapshotStateList<Any>) {
    Column(modifier = Modifier.scrollable(rememberScrollState(), Orientation.Vertical)) {
        Text("Firmware info")
        Text("Firmware update chooser")
    }
}