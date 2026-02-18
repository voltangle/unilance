package com.arvenora.lancemate.ui

import androidx.compose.animation.AnimatedContent
import androidx.compose.animation.AnimatedVisibility
import androidx.compose.animation.core.animateFloatAsState
import androidx.compose.foundation.layout.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.graphicsLayer
import androidx.compose.ui.semantics.contentDescription
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.semantics.stateDescription
import androidx.compose.ui.unit.dp
import com.arvenora.lancemate.viewmodel.ConnectionManagerViewModel
import com.arvenora.lancemate.viewmodel.ConnectionState
import com.arvenora.lancemate.viewmodel.DeviceConnectionType
import lancemate.composeapp.generated.resources.*
import org.jetbrains.compose.resources.painterResource

@OptIn(ExperimentalMaterial3ExpressiveApi::class, ExperimentalMaterial3Api::class)
@Composable
fun FAB(
    modifier: Modifier = Modifier,
    expanded: Boolean,
    inNavRail: Boolean,
    viewModel: ConnectionManagerViewModel
) {
    // It could be that all of these iterator calls on viewModel.devices can seriously
    // degrade performance, but idgaf if it doesn't and works good enough, I don't care
    // enough to make it as light as possible, I need it to work and work reliably
    val connected = viewModel.devices.any { it.state == ConnectionState.Connected }
    if (inNavRail) {
        ExtendedFloatingActionButton(
            modifier = modifier,
            elevation = FloatingActionButtonDefaults.elevation(
                0.dp, 0.dp, 0.dp, 0.dp
            ),
            expanded = expanded,
            onClick = { viewModel.showConnectionSheet = true },
            icon = {
                MainIcon(connected)
            },
            text = {
                FABText(connected)
            })
    } else {
        Box(modifier = Modifier.wrapContentSize()) {
            var checked by remember { mutableStateOf(false) }
            SplitButtonLayout(leadingButton = {
                SplitButtonDefaults.ElevatedLeadingButton(
                    modifier = Modifier.height(56.dp), // FAB baseline token
                    onClick = {
                        viewModel.showConnectionSheet = true
                    },
                    colors = ButtonDefaults.filledTonalButtonColors(
                        containerColor = FloatingActionButtonDefaults.containerColor,
                        contentColor = contentColorFor(
                            FloatingActionButtonDefaults.containerColor
                        )
                    ),
                    shapes = if (connected) SplitButtonDefaults.leadingButtonShapesFor(
                        56.dp // FAB baseline token
                    ) else SplitButtonShapes(
                        FloatingActionButtonDefaults.mediumShape, null, null
                    ),
                    contentPadding = SplitButtonDefaults.trailingButtonContentPaddingFor(
                        56.dp
                    )
                ) {
                    MainIcon(connected)
                    AnimatedVisibility(expanded) {
                        Row {
                            Spacer(Modifier.size(ButtonDefaults.IconSpacing))
                            FABText(connected)
                        }
                    }
                }
            }, trailingButton = {
                AnimatedVisibility(
                    connected, Modifier.height(56.dp)
                ) {
                    val description = "See connected devices"

                    TooltipBox(
                        positionProvider = TooltipDefaults.rememberTooltipPositionProvider(
                            TooltipAnchorPosition.Above
                        ),
                        tooltip = { PlainTooltip { Text(description) } },
                        state = rememberTooltipState(),
                    ) {
                        SplitButtonDefaults.ElevatedTrailingButton(
                            checked = checked,
                            onCheckedChange = { checked = it },
                            shapes = SplitButtonDefaults.trailingButtonShapesFor(
                                56.dp // FAB baseline token
                            ),
                            colors = ButtonDefaults.filledTonalButtonColors(
                                containerColor = FloatingActionButtonDefaults.containerColor,
                                contentColor = contentColorFor(
                                    FloatingActionButtonDefaults.containerColor
                                )
                            ),
                            contentPadding = SplitButtonDefaults.trailingButtonContentPaddingFor(
                                56.dp
                            ),
                            modifier = Modifier.semantics {
                                stateDescription =
                                    if (checked) "Expanded" else "Collapsed"
                                contentDescription = description
                            }.height(56.dp).widthIn(min = 56.dp), // FAB baseline token
                        ) {
                            val rotation: Float by animateFloatAsState(
                                targetValue = if (checked) 180f else 0f,
                                label = "Trailing Icon Rotation",
                            )
                            Icon(
                                painterResource(Res.drawable.arrow_drop_up),
                                modifier = Modifier.size(SplitButtonDefaults.TrailingIconSize)
                                    .graphicsLayer {
                                        this.rotationZ = rotation
                                    },
                                contentDescription = "Connected devices list",
                            )
                        }
                    }
                }
            })

            DropdownMenu(expanded = checked, onDismissRequest = { checked = false }) {
                viewModel.devices.forEach {
                    DropdownMenuItem(
                        text = { Text(it.name) },
                        onClick = { /* Handle press! */ },
                        leadingIcon = {
                            Icon(
                                painterResource(
                                    when (it.connection) {
                                        is DeviceConnectionType.Bluetooth -> Res.drawable.bluetooth
                                        is DeviceConnectionType.TCP -> Res.drawable.cloud
                                        is DeviceConnectionType.USB -> Res.drawable.usb
                                    }
                                ), contentDescription = null
                            )
                        },
                    )
                }
            }
        }
    }
}

@Composable
private fun MainIcon(connection: Boolean) {
    AnimatedContent(connection) {
        if (it) {
            Icon(
                painterResource(Res.drawable.link_2), "Connected"
            )
        } else {
            Icon(
                painterResource(Res.drawable.link_off), "Not connected"
            )
        }
    }
}

@Composable
private fun FABText(connected: Boolean) {
    AnimatedContent(connected) {
        if (it) {
            Text("Connected")
        } else {
            Text("Connect")
        }
    }
}