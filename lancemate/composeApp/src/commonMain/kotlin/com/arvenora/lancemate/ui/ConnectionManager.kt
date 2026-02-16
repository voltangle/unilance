package com.arvenora.lancemate.ui

import androidx.compose.animation.AnimatedContent
import androidx.compose.animation.AnimatedVisibility
import androidx.compose.animation.animateContentSize
import androidx.compose.animation.core.Spring
import androidx.compose.animation.core.spring
import androidx.compose.animation.core.tween
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.rememberLazyListState
import androidx.compose.material3.*
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
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
        LaunchedEffect(viewModel.connectionState) {
            if (viewModel.connectionState == ConnectionState.Connected) {
                listState.animateScrollToItem(0)
            }
        }
        AnimatedVisibility(viewModel.connectionState == ConnectionState.Connecting) {
            LinearWavyProgressIndicator(modifier = Modifier.fillMaxWidth())
        }
        AnimatedVisibility(viewModel.connectionState == ConnectionState.Connected) {
            Column {
                Text(
                    text = "Connected devices",
                    style = MaterialTheme.typography.titleSmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                    modifier = Modifier.padding(bottom = 8.dp, start = 8.dp)
                )
                SegmentedListItem(
                    leadingContent = {
                        Icon(
                            painterResource(Res.drawable.bluetooth),
                            contentDescription = "Localized description",
                            modifier = Modifier.padding(start = 8.dp),
                        )
                    },
                    colors = ListItemDefaults.colors(containerColor = MaterialTheme.colorScheme.inverseOnSurface),
                    shapes = ListItemDefaults.segmentedShapes(0, 0),
                    selected = true,
                    onClick = {
                        scope.launch {
                            viewModel.disconnectFromBleDevice()
                        }
                    },
                    content = {
                        viewModel.connectedDevice?.let {
                            Text(
                                it, fontSize = 20.sp
                            )
                        }
                    },
                )
            }
        }
        Row(
            Modifier.padding(horizontal = 8.dp).fillMaxWidth(),
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
        AnimatedContent(viewModel.method) {
            when (it) {
                ConnectionMethod.BLE -> {
                    Column {
                        LazyColumn(
                            modifier = Modifier.clip(ListItemDefaults.shapes().selectedShape)
                                .animateContentSize(),
                            state = listState,
                            verticalArrangement = Arrangement.spacedBy(8.dp)
                        ) {
                            item(key = "devices") {
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
                                    viewModel.bleDevices.forEachIndexed { index, device ->
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
                                                containerColor = MaterialTheme.colorScheme.inverseOnSurface
                                            ),
                                            shapes = ListItemDefaults.segmentedShapes(
                                                index = index,
                                                count = viewModel.bleDevices.size
                                            ),
                                            selected = viewModel.connectedDevice == device,
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