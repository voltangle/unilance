package com.arvenora.lancemate

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.isSystemInDarkTheme
import androidx.compose.material3.*
import androidx.compose.material3.adaptive.currentWindowAdaptiveInfo
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.semantics.stateDescription
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.window.core.layout.WindowSizeClass
import kotlinx.coroutines.launch
import lancemate.composeapp.generated.resources.*
import org.jetbrains.compose.resources.painterResource

@OptIn(ExperimentalMaterial3Api::class)
@Composable
@Preview
fun App() {
    var selectedItem by remember { mutableIntStateOf(0) }
    val items = listOf("Home", "Search", "Settings")
    val selectedIcons = listOf(Res.drawable.home, Res.drawable.favorite, Res.drawable.star)
    val unselectedIcons = listOf(Res.drawable.home, Res.drawable.favorite, Res.drawable.star)
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

    MaterialTheme(colorScheme = colors) {
            if (sizeClass.isWidthAtLeastBreakpoint(WindowSizeClass.WIDTH_DP_MEDIUM_LOWER_BOUND) || !sizeClass.isHeightAtLeastBreakpoint(
                    WindowSizeClass.HEIGHT_DP_MEDIUM_LOWER_BOUND)) {
                Scaffold { contentPadding ->
                    Row(Modifier.fillMaxWidth().padding(contentPadding)) {
                        WideNavigationRail(
                            state = state,
                            header = {
                                // Header icon button should have a tooltip.
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
                            },
                        ) {
                            items.forEachIndexed { index, item ->
                                WideNavigationRailItem(
                                    railExpanded = state.targetValue == WideNavigationRailValue.Expanded,
                                    icon = {
                                        Icon(
                                            painterResource(
                                                if (selectedItem == index) selectedIcons[index]
                                                else unselectedIcons[index]
                                            ),
                                            contentDescription = item,
                                        )
                                    },
                                    label = { Text(item) },
                                    selected = selectedItem == index,
                                    onClick = { selectedItem = index },
                                )
                            }
                        }

                        val textString =
                            if (state.currentValue == WideNavigationRailValue.Expanded) {
                                "Expanded"
                            } else {
                                "Collapsed"
                            }
                        Column {
                            Text(modifier = Modifier.padding(16.dp), text = "The rail is $textString.")
                            Text(
                                modifier = Modifier.padding(16.dp),
                                text =
                                    "Note: This demo is best shown in portrait mode, as landscape mode" +
                                            " may result in a compact height in certain devices. For any" +
                                            " compact screen dimensions, use a Navigation Bar instead.",
                            )
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
                                            if (selectedItem == index) selectedIcons[index]
                                            else unselectedIcons[index]
                                        ),
                                        contentDescription = item,
                                    )
                                },
                                label = { Text(item) },
                                selected = selectedItem == index,
                                onClick = { selectedItem = index },
                            )
                        }
                    }
                }) { contentPadding ->
                    Row(modifier = Modifier.padding(contentPadding)) {
                        Text("some content")
                    }
                }
            }
    }
}
