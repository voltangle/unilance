package com.arvenora.lancemate.ui

import androidx.compose.animation.AnimatedContent
import androidx.compose.animation.AnimatedVisibility
import androidx.compose.animation.animateContentSize
import androidx.compose.animation.core.Spring
import androidx.compose.animation.core.spring
import androidx.compose.animation.core.tween
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.lazy.itemsIndexed
import androidx.compose.foundation.lazy.rememberLazyListState
import androidx.compose.material3.*
import androidx.compose.runtime.Composable
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
import lancemate.composeapp.generated.resources.*
import org.jetbrains.compose.resources.painterResource

@OptIn(ExperimentalMaterial3ExpressiveApi::class)
@Composable
fun ConnectionManager(viewModel: ConnectionManagerViewModel, scope: CoroutineScope) {
    Column(
        modifier = Modifier.fillMaxWidth(),
        verticalArrangement = Arrangement.spacedBy(8.dp)
    ) {
        val options = ConnectionMethod.entries.toTypedArray()
        val icons = listOf(
            Res.drawable.bluetooth,
            Res.drawable.cloud,
            Res.drawable.usb,
        )

        Text(
            "Connection manager", style = MaterialTheme.typography.displaySmallEmphasized
        )
        val listState = rememberLazyListState()
        AnimatedVisibility(viewModel.showLoading) {
            LinearWavyProgressIndicator(modifier = Modifier.fillMaxWidth())
        }
        AnimatedContent(viewModel.anyConnectedDevices) {
            if (it) {
                Column {
                    Text(
                        text = "Connected devices",
                        style = MaterialTheme.typography.titleSmall,
                        color = MaterialTheme.colorScheme.onSurfaceVariant,
                        modifier = Modifier.padding(bottom = 8.dp, start = 8.dp)
                    )
                    LazyColumn {
                        items(viewModel.devices) { device ->
                            SegmentedListItem(
                                leadingContent = {
                                    Icon(
                                        painterResource(Res.drawable.bluetooth),
                                        contentDescription = "Localized description",
                                        modifier = Modifier.padding(start = 8.dp),
                                        tint = MaterialTheme.colorScheme.onPrimaryContainer
                                    )
                                },
                                trailingContent = {
                                    IconButton(onClick = {
                                        scope.launch {
                                            viewModel.disconnectFromDevice(device)
                                        }
                                    }) {
                                        Icon(painterResource(Res.drawable.close), "")
                                    }
                                },
                                colors = ListItemDefaults.colors(containerColor = MaterialTheme.colorScheme.primaryContainer),
                                shapes = ListItemDefaults.segmentedShapes(0, 0),
                                selected = true,
                                onClick = {
                                    // TODO: this should probably select the currently "active"
                                    // device, aka the one we're communicating with right now
                                },
                                content = {
                                    Text(
                                        device.name,
                                        fontSize = 20.sp,
                                        color = MaterialTheme.colorScheme.onPrimaryContainer
                                    )
                                },
                            )
                        }
                    }
                }
            } else {
                Text(
                    text = "Not connected to any device",
                    style = MaterialTheme.typography.titleSmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                    modifier = Modifier.padding(bottom = 8.dp, start = 8.dp)
                )
            }
        }
        Row(
            Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.spacedBy(ButtonGroupDefaults.ConnectedSpaceBetween),
        ) {
            options.forEachIndexed { index, method ->
                ToggleButton(
                    checked = viewModel.method == method,
                    onCheckedChange = { viewModel.method = method },
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
        AnimatedContent(viewModel.method) { method ->
            when (method) {
                ConnectionMethod.BLE -> {
                    Column(
                        verticalArrangement = Arrangement.spacedBy(
                            ListItemDefaults.SegmentedGap
                        ),
                    ) {
                        Text(
                            text = "Devices",
                            style = MaterialTheme.typography.titleSmall,
                            color = MaterialTheme.colorScheme.onSurfaceVariant,
                            modifier = Modifier.padding(
                                bottom = 8.dp, start = 8.dp
                            )
                        )
                        LazyColumn(
                            modifier = Modifier.clip(ListItemDefaults.shapes().selectedShape)
                                .animateContentSize(),
                            state = listState,
                            verticalArrangement = Arrangement.spacedBy(
                                ListItemDefaults.SegmentedGap
                            ),
                        ) {
                            itemsIndexed(viewModel.devices) { index, device ->
                                if (device.state != ConnectionState.Connected) {
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
                                        colors = ListItemDefaults.colors(
                                            // why the fuck do I need to make it inverse bro
                                            containerColor = MaterialTheme.colorScheme.inverseOnSurface
                                        ),
                                        shapes = ListItemDefaults.segmentedShapes(
                                            // I agree, this is kinda a bug. When not all
                                            // devices are "connected" devices (and almost
                                            // always it is), it will do wrong shapes.
                                            // But consider this, the LazyList itself
                                            // also has a clip with the exact same
                                            // shape as this guy, so the list will look
                                            // correct anyway
                                            index = index,
                                            count = viewModel.devices.size
                                        ),
                                        selected = false,
                                        onClick = {
                                            scope.launch {
                                                viewModel.connectToDevice(device)
                                            }
                                        },
                                        content = {
                                            Text(
                                                device.name, fontSize = 20.sp
                                            )
                                        },
                                    )
                                }
                            }
                        }
                    }
                }

                ConnectionMethod.TCP -> {
                    Column(modifier = Modifier.fillMaxWidth()) {
                        Text("TODO: TCP")
                    }
                }

                ConnectionMethod.USB -> {
                    Column(modifier = Modifier.fillMaxWidth()) {
                        Text("TODO: USB")
                    }
                }
            }
        }
    }
}