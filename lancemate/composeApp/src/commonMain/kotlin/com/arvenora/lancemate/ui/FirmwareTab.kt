package com.arvenora.lancemate.ui

import androidx.compose.foundation.gestures.Orientation
import androidx.compose.foundation.gestures.scrollable
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.material3.Card
import androidx.compose.material3.CardDefaults
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.snapshots.SnapshotStateList
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp

@Composable
fun FirmwareTab(isExpanded: Boolean, backStack: SnapshotStateList<Any>) {
    Card(
        colors = CardDefaults.cardColors(
            containerColor = MaterialTheme.colorScheme.background,
        ),
        modifier = if (isExpanded) Modifier.fillMaxSize()
            .padding(bottom = 20.dp, end = 20.dp) else Modifier.fillMaxSize()
    ) {
        Column(
            modifier = Modifier.scrollable(
                rememberScrollState(), Orientation.Vertical
            )
        ) {
            Text("Firmware info")
            Text("Firmware update chooser")
        }
    }
}