package com.arvenora.lancemate.ui

import androidx.compose.animation.AnimatedContent
import androidx.compose.animation.AnimatedVisibility
import androidx.compose.animation.animateContentSize
import androidx.compose.animation.core.Spring
import androidx.compose.animation.core.animateFloatAsState
import androidx.compose.animation.core.spring
import androidx.compose.animation.core.tween
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.itemsIndexed
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.semantics.role
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.arvenora.lancemate.viewmodel.ConnectionManagerViewModel
import com.arvenora.lancemate.viewmodel.ConnectionMethod
import com.arvenora.lancemate.viewmodel.ConnectionState
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.launch
import lancemate.composeapp.generated.resources.Res
import lancemate.composeapp.generated.resources.bluetooth
import lancemate.composeapp.generated.resources.cloud
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
            Res.drawable.cloud,
            Res.drawable.usb,
        )

        AnimatedContent(viewModel.connectionState.value) {
            Text(
                modifier = Modifier.padding(8.dp), text = when (it) {
                    ConnectionState.NotConnected -> "Not connected"
                    ConnectionState.Connecting -> "Connecting..."
                    ConnectionState.Connected -> "Connected: ${viewModel.connectedDevice.value}"
                }, style = MaterialTheme.typography.displaySmallEmphasized
            )
        }
        AnimatedVisibility(viewModel.connectionState.value == ConnectionState.Connecting) {
            LinearWavyProgressIndicator(modifier = Modifier.fillMaxWidth())
        }
        Row(
            Modifier.padding(horizontal = 8.dp).fillMaxWidth(),
            horizontalArrangement = Arrangement.spacedBy(ButtonGroupDefaults.ConnectedSpaceBetween),
        ) {
            options.forEachIndexed { index, method ->
                ToggleButton(
                    checked = viewModel.method.value == method,
                    onCheckedChange = { viewModel.setMethod(method) },
                    modifier = Modifier.semantics { role = Role.RadioButton }.weight(1f),
                    shapes = when (index) {
                        0 -> ButtonGroupDefaults.connectedLeadingButtonShapes()
                        options.lastIndex -> ButtonGroupDefaults.connectedTrailingButtonShapes()
                        else -> ButtonGroupDefaults.connectedMiddleButtonShapes()
                    },
                ) {
                    Icon(painterResource(icons[index]), "")
                    Spacer(Modifier.size(ToggleButtonDefaults.IconSpacing))
                    Text(method.name)
                }
            }
        }
        AnimatedVisibility(viewModel.method.value == ConnectionMethod.BLE) {
            LazyColumn(
                modifier = Modifier.clip(ListItemDefaults.shapes().selectedShape)
                    .animateContentSize(),
                verticalArrangement = Arrangement.spacedBy(ListItemDefaults.SegmentedGap),
            ) {
                itemsIndexed(viewModel.bleDevices) { index, device ->
                    SegmentedListItem(
                        modifier = Modifier.animateItem(
                            fadeInSpec = tween(durationMillis = 250),
                            fadeOutSpec = tween(durationMillis = 250),
                            placementSpec = spring(
                                stiffness = Spring.StiffnessLow,
                                dampingRatio = Spring.DampingRatioMediumBouncy
                            )
                        ),
                        leadingContent = {
                            Icon(
                                painterResource(Res.drawable.bluetooth),
                                contentDescription = "Localized description",
                                modifier = Modifier.padding(start = 8.dp),
                            )
                        },
                        colors = ListItemDefaults.colors(containerColor = MaterialTheme.colorScheme.inverseOnSurface),
                        shapes = ListItemDefaults.segmentedShapes(
                            index = index, count = viewModel.bleDevices.size
                        ),
                        selected = viewModel.connectedDevice.value == device,
                        onClick = {
                            scope.launch {
                                viewModel.connectToBleDevice(device)
                            }
                        },
                        content = { Text(device, fontSize = 20.sp) },
                    )
                }
            }
        }
        AnimatedVisibility(viewModel.method.value == ConnectionMethod.TCP) {
            Text("TODO: TCP")
        }
        AnimatedVisibility(viewModel.method.value == ConnectionMethod.USB) {
            Text("TODO: USB")
        }
    }
}