package com.arvenora.lancemate

import androidx.compose.animation.*
import androidx.compose.animation.core.tween
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.*
import androidx.compose.material3.adaptive.currentWindowAdaptiveInfo
import androidx.compose.runtime.Composable
import androidx.compose.runtime.mutableStateListOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.runtime.snapshots.SnapshotStateList
import androidx.compose.ui.Modifier
import androidx.compose.ui.input.nestedscroll.nestedScroll
import androidx.compose.ui.platform.LocalLayoutDirection
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.compose.ui.window.Dialog
import androidx.compose.ui.zIndex
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.window.core.layout.WindowSizeClass
import com.arvenora.lancemate.nav.Root
import com.arvenora.lancemate.platform.platformSystemColorScheme
import com.arvenora.lancemate.ui.*
import com.arvenora.lancemate.viewmodel.AppViewModel
import com.arvenora.lancemate.viewmodel.ConnectionManagerViewModel
import com.arvenora.lancemate.viewmodel.ConnectionState
import kotlinx.coroutines.launch
import lancemate.composeapp.generated.resources.*
import org.jetbrains.compose.resources.painterResource

enum class RootNavTarget {
    LiveData, Config, Filesystem, CORElink, Firmware
}

@OptIn(ExperimentalMaterial3Api::class, ExperimentalMaterial3ExpressiveApi::class)
@Composable
@Preview
fun App() {
    val viewModel = viewModel { AppViewModel() }
    val connectionManagerVM =
        viewModel { ConnectionManagerViewModel(viewModel.snackbarHostState) }

    val navItems = listOf(
        Triple(RootNavTarget.LiveData, Res.drawable.ssid_chart, Res.drawable.ssid_chart),
        Triple(
            RootNavTarget.Config, Res.drawable.settings, Res.drawable.settings_fill
        ),
        Triple(RootNavTarget.Filesystem, Res.drawable.files, Res.drawable.files_fill),
        Triple(RootNavTarget.CORElink, Res.drawable.cable, Res.drawable.cable),
        Triple(RootNavTarget.Firmware, Res.drawable.memory, Res.drawable.memory_fill),
    )
    val scope = rememberCoroutineScope()
    val sizeClass = currentWindowAdaptiveInfo().windowSizeClass
    val connectionSheetState = rememberModalBottomSheetState(skipPartiallyExpanded = true)

    val backStacks = listOf(
        Pair(RootNavTarget.LiveData, remember { mutableStateListOf<Any>(Root) }),
        Pair(RootNavTarget.Config, remember { mutableStateListOf<Any>(Root) }),
        Pair(RootNavTarget.Filesystem, remember { mutableStateListOf<Any>(Root) }),
        Pair(RootNavTarget.CORElink, remember { mutableStateListOf<Any>(Root) }),
        Pair(RootNavTarget.Firmware, remember { mutableStateListOf<Any>(Root) }),
    )

    val navigationRailState = rememberWideNavigationRailState()
    val headerDescription =
        if (navigationRailState.targetValue == WideNavigationRailValue.Expanded) {
            "Collapse rail"
        } else {
            "Expand rail"
        }
    val isExpandedSize =
        sizeClass.isWidthAtLeastBreakpoint(WindowSizeClass.WIDTH_DP_MEDIUM_LOWER_BOUND) && sizeClass.isHeightAtLeastBreakpoint(
            WindowSizeClass.HEIGHT_DP_MEDIUM_LOWER_BOUND
        )


    MaterialExpressiveTheme(colorScheme = platformSystemColorScheme()) {
        Scaffold(
            containerColor = MaterialTheme.colorScheme.surfaceContainer, snackbarHost = {
                SnackbarHost(
                    hostState = viewModel.snackbarHostState,
                    modifier = Modifier.zIndex(1000f)
                )
            }) { contentPadding ->
            Row(
                Modifier.fillMaxSize().padding(
                    start = contentPadding.calculateLeftPadding(
                        LocalLayoutDirection.current
                    ), end = contentPadding.calculateRightPadding(
                        LocalLayoutDirection.current
                    )
                )
            ) {
                if (isExpandedSize) {
                    WideNavigationRail(
                        modifier = Modifier.wrapContentWidth().widthIn(min = 94.dp),
                        colors = WideNavigationRailDefaults.colors(containerColor = MaterialTheme.colorScheme.surfaceContainer),
                        state = navigationRailState,
                        header = {
                            Column {
                                TooltipBox(
                                    positionProvider = TooltipDefaults.rememberTooltipPositionProvider(
                                        TooltipAnchorPosition.End
                                    ),
                                    tooltip = { PlainTooltip { Text(headerDescription) } },
                                    state = rememberTooltipState(),
                                ) {
                                    IconButton(
                                        modifier = Modifier.padding(
                                            start = 24.dp, end = 24.dp
                                        ),
                                        onClick = {
                                            scope.launch {
                                                if (navigationRailState.targetValue == WideNavigationRailValue.Expanded) navigationRailState.collapse()
                                                else navigationRailState.expand()
                                            }
                                        },
                                    ) {
                                        if (navigationRailState.targetValue == WideNavigationRailValue.Expanded) {
                                            Icon(
                                                painterResource(Res.drawable.menu_open),
                                                headerDescription
                                            )
                                        } else {
                                            Icon(
                                                painterResource(Res.drawable.menu),
                                                headerDescription
                                            )
                                        }
                                    }
                                }
                                TooltipBox(
                                    positionProvider = TooltipDefaults.rememberTooltipPositionProvider(
                                        TooltipAnchorPosition.End
                                    ),
                                    tooltip = { PlainTooltip { Text("Manage current connection") } },
                                    state = rememberTooltipState(),
                                ) {
                                    FAB(
                                        modifier = Modifier.padding(
                                            start = 20.dp, end = 20.dp
                                        ),
                                        navigationRailState.targetValue == WideNavigationRailValue.Expanded,
                                        true,
                                        connectionManagerVM,
                                    )
                                }
                            }
                        },
                    ) {
                        navItems.forEachIndexed { index, item ->
                            WideNavigationRailItem(
                                railExpanded = navigationRailState.targetValue == WideNavigationRailValue.Expanded,
                                icon = {
                                    Icon(
                                        painterResource(
                                            if (viewModel.rootNavTarget.ordinal == index) item.third
                                            else item.second
                                        ),
                                        contentDescription = item.first.name,
                                    )
                                },
                                label = { Text(item.first.name) },
                                selected = viewModel.rootNavTarget.ordinal == index,
                                onClick = { viewModel.rootNavTarget = item.first },
                            )
                        }
                    }
                }

                val scrollBehavior =
                    TopAppBarDefaults.enterAlwaysScrollBehavior(rememberTopAppBarState())

                Scaffold(
                    containerColor = if (isExpandedSize) MaterialTheme.colorScheme.surfaceContainer else MaterialTheme.colorScheme.background,
                    // this is a bit cursed, but its so scrolling doesn't break with a small top bar
                    modifier = if (!isExpandedSize) Modifier.nestedScroll(
                        scrollBehavior.nestedScrollConnection
                    ) else Modifier, bottomBar = {
                        if (!isExpandedSize) {
                            NavigationBar(windowInsets = NavigationBarDefaults.windowInsets) {
                                navItems.forEachIndexed { index, item ->
                                    NavigationBarItem(
                                        icon = {
                                            Icon(
                                                painterResource(
                                                    if (viewModel.rootNavTarget.ordinal == index) item.third
                                                    else item.second
                                                ),
                                                contentDescription = item.first.name,
                                            )
                                        },
                                        label = { Text(item.first.name) },
                                        selected = viewModel.rootNavTarget.ordinal == index,
                                        onClick = {
                                            viewModel.rootNavTarget = item.first
                                        },
                                    )
                                }
                            }
                        }
                    }, floatingActionButton = {
                        if (!isExpandedSize) {
                            TooltipBox(
                                positionProvider = TooltipDefaults.rememberTooltipPositionProvider(
                                    TooltipAnchorPosition.Above
                                ),
                                tooltip = { PlainTooltip { Text("Manage current connection") } },
                                state = rememberTooltipState(),
                            ) {
                                FAB(
                                    expanded = !connectionManagerVM.anyConnectedDevices,
                                    inNavRail = false,
                                    viewModel = connectionManagerVM,
                                )
                            }
                        }
                    }, topBar = {
                        TopBar(
                            isExpandedSize,
                            scrollBehavior,
                            backStacks.first { v -> v.first == viewModel.rootNavTarget })
                    }) { contentPadding ->
                    Box(modifier = Modifier.padding(contentPadding)) {
                        AppContent(isExpandedSize, viewModel.rootNavTarget, backStacks)
                    }
                    if (isExpandedSize) {
                        if (connectionManagerVM.showConnectionSheet) {
                            Dialog(onDismissRequest = { connectionManagerVM.hideSheet() }) {
                                Card(
                                    modifier = Modifier.fillMaxWidth().padding(16.dp),
                                    colors = CardDefaults.cardColors(containerColor = MaterialTheme.colorScheme.surfaceContainerLowest),
                                    shape = RoundedCornerShape(16.dp),
                                ) {
                                    Box(modifier = Modifier.padding(16.dp)) {
                                        ConnectionManager(connectionManagerVM, scope)
                                    }
                                }
                            }
                        }
                    } else {
                        if (connectionManagerVM.showConnectionSheet) {
                            ModalBottomSheet(
                                onDismissRequest = { connectionManagerVM.hideSheet() },
                                sheetState = connectionSheetState
                            ) {
                                Box(modifier = Modifier.padding(8.dp)) {
                                    ConnectionManager(connectionManagerVM, scope)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

@Composable
fun AppContent(
    isExpanded: Boolean,
    selectedItem: RootNavTarget,
    backStacks: List<Pair<RootNavTarget, SnapshotStateList<Any>>>
) {
    // This is done as a lambda opposed to just a saved value so that the current backstack
    // is computed at the exact moment the needed tab is shown. The app crashed once
    // when changing tabs from Config to Live Data with an error "Unknown display
    // ConfigDetail(...)"  or something, which meant that a race condition somehow occurred.
    // This approach should *probably* protect from such a problem.
    val currentBackStack = {
        backStacks.first { value -> value.first == selectedItem }.second
    }
    AnimatedContent(
        selectedItem,
        // Original transition spec, just with some variables changed around
        transitionSpec = {
            (fadeIn(animationSpec = tween(200, delayMillis = 90)) + scaleIn(
                initialScale = 0.96f, animationSpec = tween(200, delayMillis = 90)
            )).togetherWith(fadeOut(animationSpec = tween(90)))
        }) {
        when (it) {
            RootNavTarget.LiveData -> {
                LiveDataTab(isExpanded, currentBackStack())
            }

            RootNavTarget.Config -> {
                ConfigTab(isExpanded, currentBackStack())
            }

            RootNavTarget.Filesystem -> {
                FilesystemTab(isExpanded, currentBackStack())
            }

            RootNavTarget.CORElink -> {
                CoreLinkTab(isExpanded, currentBackStack())
            }

            RootNavTarget.Firmware -> {
                FirmwareTab(isExpanded, currentBackStack())
            }
        }
    }
}
