package com.arvenora.lancemate.ui

import androidx.compose.foundation.gestures.Orientation
import androidx.compose.foundation.gestures.scrollable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.grid.GridCells
import androidx.compose.foundation.lazy.grid.LazyVerticalGrid
import androidx.compose.foundation.lazy.grid.items
import androidx.compose.foundation.rememberScrollState
import androidx.compose.material3.*
import androidx.compose.runtime.Composable
import androidx.compose.runtime.snapshots.SnapshotStateList
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import lancemate.composeapp.generated.resources.Res
import lancemate.composeapp.generated.resources.dashboard
import org.jetbrains.compose.resources.painterResource

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
            modifier = Modifier.fillMaxWidth().scrollable(
                rememberScrollState(), Orientation.Vertical
            ).padding(8.dp), verticalArrangement = Arrangement.spacedBy(8.dp)
        ) {
            LazyVerticalGrid(
                modifier = Modifier.fillMaxWidth(),
                columns = GridCells.Adaptive(150.dp),
                verticalArrangement = Arrangement.spacedBy(8.dp),
                horizontalArrangement = Arrangement.spacedBy(8.dp)
            ) {
                items(listOf("","","","")) {
                    DataCard("Firmware version", "0.1.4") {
                        Icon(painterResource(Res.drawable.dashboard), "")
                    }
                }
            }
        }
    }
}

@OptIn(ExperimentalMaterial3ExpressiveApi::class)
@Composable
fun DataCard(title: String, subtitle: String, icon: @Composable (() -> Unit)) {
    Card {
        Row(verticalAlignment = Alignment.CenterVertically) {
            Box(modifier = Modifier.padding(start = 16.dp)) {
                icon()
            }
            Column(
                modifier = Modifier.padding(16.dp),
                verticalArrangement = Arrangement.spacedBy(6.dp)
            ) {
                Text(title, style = MaterialTheme.typography.titleMediumEmphasized)
                Text(subtitle, style = MaterialTheme.typography.bodyMedium)
            }
        }
    }
}