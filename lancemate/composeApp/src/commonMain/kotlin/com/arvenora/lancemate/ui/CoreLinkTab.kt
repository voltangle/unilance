package com.arvenora.lancemate.ui

import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Card
import androidx.compose.material3.CardDefaults
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.snapshots.SnapshotStateList
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp

@Composable
fun CoreLinkTab(isExpanded: Boolean, backStack: SnapshotStateList<Any>) {
    Card(
        colors = CardDefaults.cardColors(
            containerColor = MaterialTheme.colorScheme.background,
        ),
        modifier = if (isExpanded) Modifier.fillMaxSize()
            .padding(bottom = 20.dp, end = 20.dp) else Modifier.fillMaxSize()
    ) {
        Text("CORElink tab")
    }
}