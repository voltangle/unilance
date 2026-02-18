package com.arvenora.lancemate.ui

import androidx.compose.animation.*
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.text.input.rememberTextFieldState
import androidx.compose.material3.*
import androidx.compose.runtime.Composable
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.runtime.snapshots.SnapshotStateList
import androidx.compose.ui.Modifier
import androidx.compose.ui.semantics.clearAndSetSemantics
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import com.arvenora.lancemate.RootNavTarget
import kotlinx.coroutines.launch
import lancemate.composeapp.generated.resources.Res
import lancemate.composeapp.generated.resources.arrow_back
import lancemate.composeapp.generated.resources.menu
import org.jetbrains.compose.resources.painterResource

@OptIn(ExperimentalMaterial3Api::class, ExperimentalMaterial3ExpressiveApi::class)
@Composable
fun TopBar(
    isExpanded: Boolean,
    scrollBehavior: TopAppBarScrollBehavior,
    backStack: Pair<RootNavTarget, SnapshotStateList<Any>>
) {
    if (isExpanded) {
        TopAppBar(
            // this is there so it matches the height of the window header draggable area
            // on macOS
            modifier = Modifier.height(54.dp),
            colors = TopAppBarDefaults.topAppBarColors(containerColor = MaterialTheme.colorScheme.surfaceContainer),
            title = {
                Text(
                    "LANCEmate", maxLines = 1, overflow = TextOverflow.Ellipsis, style = MaterialTheme.typography.titleLargeEmphasized
                )
            }, navigationIcon = {
                BackButton(backStack.second)
            }, actions = {
                IconButton(onClick = { /* do something */ }) {
                    Icon(
                        painterResource(Res.drawable.menu),
                        contentDescription = "Localized description"
                    )
                }
            })
    } else {
        // Config tab has a custom top app bar
        if (backStack.first != RootNavTarget.Config) {
            MediumTopAppBar(
                scrollBehavior = scrollBehavior, title = {
                    Text(
                        "LANCEmate", maxLines = 1, overflow = TextOverflow.Ellipsis, style = MaterialTheme.typography.headlineMediumEmphasized
                    )
                }, navigationIcon = {
                    BackButton(backStack.second)
                }, actions = {
                    IconButton(onClick = { /* do something */ }) {
                        Icon(
                            painterResource(Res.drawable.menu),
                            contentDescription = "Localized description"
                        )
                    }
                })
        }
    }
}

@Composable
private fun BackButton(backStack: SnapshotStateList<Any>) {
    AnimatedVisibility(
        // if at least one backstack is not empty
        backStack.size > 1,
        enter = fadeIn() + expandHorizontally(),
        exit = fadeOut() + shrinkHorizontally()
    ) {
        IconButton(onClick = {
            if (backStack.size > 1) {
                backStack.removeLastOrNull()
            }
        }) {
            Icon(
                painterResource(Res.drawable.arrow_back),
                contentDescription = "Localized description"
            )
        }
    }
}