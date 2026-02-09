package com.arvenora.lancemate.ui

import androidx.compose.animation.AnimatedContent
import androidx.compose.animation.AnimatedVisibility
import androidx.compose.animation.animateContentSize
import androidx.compose.animation.core.Spring
import androidx.compose.animation.core.spring
import androidx.compose.animation.core.tween
import androidx.compose.animation.slideInVertically
import androidx.compose.animation.slideOutVertically
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.*
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.arvenora.lancemate.viewmodel.ConnectionManagerViewModel
import com.arvenora.lancemate.viewmodel.ConnectionMethod
import com.arvenora.lancemate.viewmodel.ConnectionState
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.launch
import lancemate.composeapp.generated.resources.Res
import lancemate.composeapp.generated.resources.bluetooth
import lancemate.composeapp.generated.resources.usb
import org.jetbrains.compose.resources.painterResource

@OptIn(ExperimentalMaterial3ExpressiveApi::class)
@Composable
fun ConnectionManager(viewModel: ConnectionManagerViewModel, scope: CoroutineScope) {
    Column(
        modifier = Modifier.fillMaxWidth().heightIn(max = 500.dp),
        verticalArrangement = Arrangement.spacedBy(8.dp)
    ) {
        val options = ConnectionMethod.entries.toTypedArray()
        val icons = listOf(
            Res.drawable.bluetooth,
            Res.drawable.usb,
        )

        AnimatedContent(viewModel.connectionState.value) {
            Text(
                modifier = Modifier.padding(8.dp),
                text = when (it) {
                    ConnectionState.NotConnected -> "Not connected"
                    ConnectionState.Connecting -> "Connecting..."
                    ConnectionState.Connected -> "Connected: ${viewModel.connectedDevice.value}"
                },
                style = MaterialTheme.typography.displaySmallEmphasized
            )
        }
        SingleChoiceSegmentedButtonRow(modifier = Modifier.fillMaxWidth()) {
            options.forEachIndexed { index, method ->
                SegmentedButton(
                    shape = SegmentedButtonDefaults.itemShape(
                        index = index, count = options.size
                    ),
                    onClick = { viewModel.setMethod(method) },
                    selected = method == viewModel.method.value,
                    label = {
                        Row {
                            Icon(painterResource(icons[index]), "")
                            Spacer(Modifier.size(ToggleButtonDefaults.IconSpacing))
                            Text(method.name)
                        }
                    })
            }
        }
        AnimatedVisibility(viewModel.method.value == ConnectionMethod.Bluetooth) {
            LazyColumn(
                modifier = Modifier.clip(RoundedCornerShape(8.dp)).animateContentSize(),
            ) {
                items(viewModel.bleDevices) { device ->
                    ListItem(
                        modifier = Modifier.clickable {
                            scope.launch {
                                viewModel.connectToBleDevice(device)
                            }
                        }.animateItem(
                            fadeInSpec = tween(durationMillis = 250),
                            fadeOutSpec = tween(durationMillis = 250),
                            placementSpec = spring(stiffness = Spring.StiffnessLow, dampingRatio = Spring.DampingRatioMediumBouncy)
                        ),
                        headlineContent = { Text(device, fontSize = 20.sp) },
                        leadingContent = {
                            Icon(
                                painterResource(Res.drawable.bluetooth),
                                contentDescription = "Localized description",
                                modifier = Modifier.padding(start = 8.dp),
                            )
                        },
                    )
                }
            }
        }
        AnimatedVisibility(viewModel.method.value == ConnectionMethod.USB) {
            Text("TODO: USB")
        }
    }
}