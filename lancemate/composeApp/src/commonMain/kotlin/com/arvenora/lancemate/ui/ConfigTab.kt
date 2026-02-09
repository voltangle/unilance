package com.arvenora.lancemate.ui

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material3.*
import androidx.compose.material3.adaptive.ExperimentalMaterial3AdaptiveApi
import androidx.compose.material3.adaptive.navigation3.ListDetailSceneStrategy
import androidx.compose.material3.adaptive.navigation3.rememberListDetailSceneStrategy
import androidx.compose.runtime.Composable
import androidx.compose.runtime.snapshots.SnapshotStateList
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import androidx.navigation3.runtime.entryProvider
import androidx.navigation3.ui.NavDisplay
import com.arvenora.lancemate.nav.ConfigDetail
import com.arvenora.lancemate.nav.Root
import io.github.koalaplot.core.util.ExperimentalKoalaPlotApi
import lancemate.composeapp.generated.resources.Res
import lancemate.composeapp.generated.resources.usb
import org.jetbrains.compose.resources.painterResource

@OptIn(
    ExperimentalKoalaPlotApi::class,
    ExperimentalMaterial3AdaptiveApi::class,
    ExperimentalMaterial3ExpressiveApi::class
)
@Composable
fun ConfigTab(isExpanded: Boolean, backStack: SnapshotStateList<Any>) {
    val strategy = rememberListDetailSceneStrategy<Any>()

    NavDisplay(
        backStack = backStack,
        onBack = { backStack.removeLastOrNull() },
        sceneStrategy = strategy,
        entryProvider = entryProvider {
            entry<Root>(
                metadata = ListDetailSceneStrategy.listPane(
                    detailPlaceholder = {
                        Column(
                            horizontalAlignment = Alignment.CenterHorizontally,
                            verticalArrangement = Arrangement.Center,
                            modifier = Modifier.fillMaxSize()
                        ) {
                            Text("Choose a item from the list screen")
                        }
                    })
            ) {
                LazyColumn {
                    items((0..4).count()) { v ->
                        ListItem(
                            modifier = Modifier.clickable {
                                if (backStack.size > 1) {
                                    backStack[1] = ConfigDetail(id = v)
                                } else {
                                    backStack.add(ConfigDetail(id = v))
                                }
                            },
                            colors = if (isExpanded) ListItemDefaults.colors(
                                containerColor = MaterialTheme.colorScheme.surfaceContainer
                            ) else ListItemDefaults.colors(),
                            headlineContent = { Text("Section $v", fontSize = 20.sp) },
                            supportingContent = { Text("Section description") },
                            leadingContent = {
                                Icon(
                                    painterResource(Res.drawable.usb),
                                    contentDescription = "Localized description",
                                    modifier = Modifier.padding(start = 8.dp),
                                )
                            },
                        )
                    }
                }
            }
            // TODO: make the animations native
            entry<ConfigDetail>(metadata = ListDetailSceneStrategy.detailPane()) { value ->
                Card(
                    colors = CardDefaults.cardColors(
                        containerColor = MaterialTheme.colorScheme.background,
                    ),
                    modifier = Modifier.fillMaxSize().padding(end = 20.dp, bottom = 20.dp)
                ) {
                    Column(
                        modifier = Modifier.padding(8.dp).fillMaxSize(),
                        verticalArrangement = Arrangement.Center,
                        horizontalAlignment = Alignment.CenterHorizontally
                    ) {
                        Text("Config section details ${value.id}")
                    }
                }
            }
        })
}


// TODO: make work when newer nav alphas release
//@OptIn(ExperimentalMaterial3AdaptiveApi::class)
//@Composable
//private fun isTwoPaneListDetail(): Boolean {
//    val sceneScope = LocalListDetailSceneScope.current
//        ?: return false // Not in a list-detail scene (or strategy not chosen)
//
//    // Depending on the exact API, you may have currentState/targetState.
//    // currentState is what you want for "what is visible right now".
//    val value = sceneScope.scaffoldTransitionScope.currentState
//
//    val listVisible = value[ListDetailPaneScaffoldRole.List] != PaneAdaptedValue.Hidden
//    val detailVisible = value[ListDetailPaneScaffoldRole.Detail] != PaneAdaptedValue.Hidden
//
//    return listVisible && detailVisible
//}