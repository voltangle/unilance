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
import lancemate.composeapp.generated.resources.*
import org.jetbrains.compose.resources.DrawableResource
import org.jetbrains.compose.resources.painterResource

data class FirmwareInfoCard(
    val title: String, val subtitle: String, val icon: DrawableResource
)

@Composable
fun FirmwareTab(isExpanded: Boolean, backStack: SnapshotStateList<Any>) {
    val cards = listOf(
        FirmwareInfoCard(
            title = "Manufacturer", subtitle = "REDSHIFT", Res.drawable.manufacturing
        ),
        FirmwareInfoCard(
            title = "Vehicle/HW",
            subtitle = "TOLLSHIFT (meerkat)",
            Res.drawable.developer_board
        ),
        FirmwareInfoCard(
            title = "Version",
            subtitle = "UniLANCE 1.0 Espresso",
            Res.drawable.deployed_code
        ),
        FirmwareInfoCard(
            title = "Release type", subtitle = "alpha-debug", Res.drawable.publish
        ),
    )
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
                items(cards) {
                    DataCard(Modifier, it.title, it.subtitle) {
                        Icon(painterResource(it.icon), "")
                    }
                }
            }
        }
    }
}

@OptIn(ExperimentalMaterial3ExpressiveApi::class)
@Composable
fun DataCard(
    modifier: Modifier = Modifier,
    title: String,
    subtitle: String,
    icon: @Composable (() -> Unit)
) {
    Card(modifier.height(105.dp)) {
        Row(
            modifier = Modifier.fillMaxHeight(),
            verticalAlignment = Alignment.CenterVertically
        ) {
            Box(modifier = Modifier.padding(start = 16.dp)) {
                icon()
            }
            Column(
                modifier = Modifier.padding(start = 16.dp),
                verticalArrangement = Arrangement.spacedBy(2.dp)
            ) {
                Text(title, style = MaterialTheme.typography.titleMediumEmphasized)
                Text(subtitle, style = MaterialTheme.typography.bodyMedium)
            }
        }
    }
}