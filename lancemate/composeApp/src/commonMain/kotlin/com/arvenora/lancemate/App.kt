package com.arvenora.lancemate

import androidx.compose.animation.AnimatedVisibility
import androidx.compose.animation.fadeIn
import androidx.compose.animation.fadeOut
import androidx.compose.foundation.isSystemInDarkTheme
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.*
import androidx.compose.material3.adaptive.currentWindowAdaptiveInfo
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.semantics.stateDescription
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.compose.ui.window.Dialog
import androidx.navigation3.runtime.entryProvider
import androidx.navigation3.ui.NavDisplay
import androidx.window.core.layout.WindowSizeClass
import kotlinx.coroutines.launch
import lancemate.composeapp.generated.resources.*
import org.jetbrains.compose.resources.painterResource

enum class RootNavTarget {
    Dashboard,
    LiveData,
    Filesystem,
    Firmware
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
@Preview
fun App() {
    var selectedItem by remember { mutableStateOf(RootNavTarget.Dashboard) }
    val items = listOf(RootNavTarget.Dashboard, RootNavTarget.LiveData, RootNavTarget.Filesystem, RootNavTarget.Firmware)
    val selectedIcons = listOf(Res.drawable.dashboard_fill, Res.drawable.ssid_chart, Res.drawable.files_fill, Res.drawable.memory_fill)
    val unselectedIcons = listOf(Res.drawable.dashboard, Res.drawable.ssid_chart, Res.drawable.files, Res.drawable.memory)
    val state = rememberWideNavigationRailState()
    val scope = rememberCoroutineScope()
    val headerDescription =
        if (state.targetValue == WideNavigationRailValue.Expanded) {
            "Collapse rail"
        } else {
            "Expand rail"
        }
    val sizeClass = currentWindowAdaptiveInfo().windowSizeClass
    val colors = if (!isSystemInDarkTheme()) lightColorScheme() else darkColorScheme()
    val connectionSheetState = rememberModalBottomSheetState()
    var showConnectionSheet by remember { mutableStateOf(false) }

    MaterialTheme(colorScheme = colors) {
        if (sizeClass.isWidthAtLeastBreakpoint(WindowSizeClass.WIDTH_DP_MEDIUM_LOWER_BOUND) || !sizeClass.isHeightAtLeastBreakpoint(
                WindowSizeClass.HEIGHT_DP_MEDIUM_LOWER_BOUND
            )
        ) {
            Scaffold { contentPadding ->
                Row(Modifier.fillMaxWidth().padding(contentPadding)) {
                    WideNavigationRail(
                        modifier = Modifier.wrapContentWidth(),
                        state = state,
                        header = {
                            // Header icon button should have a tooltip.
                            Column() {
                                TooltipBox(
                                    positionProvider =
                                        TooltipDefaults.rememberTooltipPositionProvider(
                                            TooltipAnchorPosition.Above
                                        ),
                                    tooltip = { PlainTooltip { Text(headerDescription) } },
                                    state = rememberTooltipState(),
                                ) {
                                    IconButton(
                                        modifier =
                                            Modifier.padding(start = 24.dp).semantics {
                                                // The button must announce the expanded or collapsed state of the
                                                // rail for accessibility.
                                                stateDescription =
                                                    if (state.currentValue == WideNavigationRailValue.Expanded) {
                                                        "Expanded"
                                                    } else {
                                                        "Collapsed"
                                                    }
                                            },
                                        onClick = {
                                            scope.launch {
                                                if (state.targetValue == WideNavigationRailValue.Expanded)
                                                    state.collapse()
                                                else state.expand()
                                            }
                                        },
                                    ) {
                                        if (state.targetValue == WideNavigationRailValue.Expanded) {
                                            Icon(painterResource(Res.drawable.menu_open), headerDescription)
                                        } else {
                                            Icon(painterResource(Res.drawable.menu), headerDescription)
                                        }
                                    }
                                }
                                TooltipBox(
                                    positionProvider =
                                        TooltipDefaults.rememberTooltipPositionProvider(TooltipAnchorPosition.Above),
                                    tooltip = { PlainTooltip { Text("Manage current connection") } },
                                    state = rememberTooltipState(),
                                ) {
                                    ExtendedFloatingActionButton(
                                        modifier = Modifier.padding(start = 20.dp),
                                        expanded = state.targetValue == WideNavigationRailValue.Expanded,
                                        onClick = { showConnectionSheet = true },
                                        icon = {
                                            Icon(painterResource(Res.drawable.link_2), "Manage current connection")
                                        },
                                        text = {
                                            Text("Connection")
                                        })
                                }
                            }
                        },
                    ) {
                        items.forEachIndexed { index, item ->
                            WideNavigationRailItem(
                                railExpanded = state.targetValue == WideNavigationRailValue.Expanded,
                                icon = {
                                    Icon(
                                        painterResource(
                                            if (selectedItem.ordinal == index) selectedIcons[index]
                                            else unselectedIcons[index]
                                        ),
                                        contentDescription = item.name,
                                    )
                                },
                                label = { Text(item.name) },
                                selected = selectedItem.ordinal == index,
                                onClick = { selectedItem = item },
                            )
                        }
                    }
                    AppContent(selectedItem)
                    AnimatedVisibility(visible = showConnectionSheet,
                        enter = fadeIn(), exit = fadeOut()
                    ) {
                        Dialog(onDismissRequest = { showConnectionSheet = false}) {
                            Card(
                                modifier = Modifier
                                    .fillMaxWidth()
                                    .height(200.dp)
                                    .padding(16.dp),
                                shape = RoundedCornerShape(16.dp),
                            ) {
                                ConnectionSheet()
                            }
                        }
                    }
                }
            }
        } else {
            Scaffold(bottomBar = {
                NavigationBar(windowInsets = NavigationBarDefaults.windowInsets) {
                    items.forEachIndexed { index, item ->
                        NavigationBarItem(
                            icon = {
                                Icon(
                                    painterResource(
                                        if (selectedItem.ordinal == index) selectedIcons[index]
                                        else unselectedIcons[index]
                                    ),
                                    contentDescription = item.name,
                                )
                            },
                            label = { Text(item.name) },
                            selected = selectedItem.ordinal == index,
                            onClick = { selectedItem = item },
                        )
                    }
                }
            }) { contentPadding ->
                AppContent(selectedItem)
                if (showConnectionSheet) {
                    ModalBottomSheet(
                        onDismissRequest = { showConnectionSheet = false },
                        sheetState = connectionSheetState
                    ) {
                        ConnectionSheet()
                    }
                }
            }
        }
    }
}

@Composable
fun AppContent(selectedItem: RootNavTarget) {
    when (selectedItem) {
        RootNavTarget.Dashboard -> {
            DashboardContent()
        }
        RootNavTarget.LiveData -> {Text("Live data")}
        RootNavTarget.Filesystem -> {Text("Filesystem")}
        RootNavTarget.Firmware -> {Text("Firmware")}
    }
}

@Composable
fun DashboardContent() {
    val backStack = remember { mutableStateListOf<Any>(Dashboard) }
    NavDisplay(
        backStack = backStack,
        onBack = { backStack.removeLastOrNull() },
        entryProvider = entryProvider {
            entry<Dashboard> {
                Text("My dashboard lol")
            }
        }
    )
}
