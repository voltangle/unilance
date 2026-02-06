package com.arvenora.lancemate

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.semantics.stateDescription
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import kotlinx.coroutines.launch
import lancemate.composeapp.generated.resources.*
import org.jetbrains.compose.resources.painterResource

@OptIn(ExperimentalMaterial3Api::class)
@Composable
@Preview
fun App() {

    var selectedItem by remember { mutableIntStateOf(0) }
    val items = listOf("Home", "Search", "Settings")
    val selectedIcons = listOf(Res.drawable.home_24px, Res.drawable.favorite_24px, Res.drawable.star_24px)
    val unselectedIcons = listOf(Res.drawable.home_24px, Res.drawable.favorite_24px, Res.drawable.star_24px)
    val state = rememberWideNavigationRailState()
    val scope = rememberCoroutineScope()
    val headerDescription =
        if (state.targetValue == WideNavigationRailValue.Expanded) {
            "Collapse rail"
        } else {
            "Expand rail"
        }

    MaterialTheme {
        Row(Modifier.fillMaxWidth()) {
            ModalWideNavigationRail(
                state = state,
                // Note: the value of expandedHeaderTopPadding depends on the layout of your screen in
                // order to achieve the best alignment.
                // expandedHeaderTopPadding = 64.dp,
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
                                Icon(painterResource(Res.drawable.menu_open_24px), headerDescription)
                            } else {
                                Icon(painterResource(Res.drawable.menu_24px), headerDescription)
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
                                painterResource(if (selectedItem == index) selectedIcons[index]
                                else unselectedIcons[index]),
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
}
