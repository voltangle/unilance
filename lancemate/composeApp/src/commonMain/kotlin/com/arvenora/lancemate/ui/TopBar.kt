package com.arvenora.lancemate.ui

import androidx.compose.animation.*
import androidx.compose.material3.*
import androidx.compose.runtime.Composable
import androidx.compose.runtime.snapshots.SnapshotStateList
import androidx.compose.ui.text.style.TextOverflow
import lancemate.composeapp.generated.resources.Res
import lancemate.composeapp.generated.resources.arrow_back
import lancemate.composeapp.generated.resources.menu
import org.jetbrains.compose.resources.painterResource

@OptIn(ExperimentalMaterial3Api::class, ExperimentalMaterial3ExpressiveApi::class)
@Composable
fun TopBar(
    isExpanded: Boolean,
    scrollBehavior: TopAppBarScrollBehavior,
    backStack: SnapshotStateList<Any>
) {
    if (isExpanded) {
        TopAppBar(
            colors = TopAppBarDefaults.topAppBarColors(containerColor = MaterialTheme.colorScheme.surfaceContainer),
            title = {
                Text(
                    "LANCEmate", maxLines = 1, overflow = TextOverflow.Ellipsis, style = MaterialTheme.typography.titleLargeEmphasized
                )
            }, navigationIcon = {
                BackButton(backStack)
            }, actions = {
                IconButton(onClick = { /* do something */ }) {
                    Icon(
                        painterResource(Res.drawable.menu),
                        contentDescription = "Localized description"
                    )
                }
            })
    } else {
        MediumTopAppBar(
            scrollBehavior = scrollBehavior, title = {
                Text(
                    "LANCEmate", maxLines = 1, overflow = TextOverflow.Ellipsis, style = MaterialTheme.typography.headlineMediumEmphasized
                )
            }, navigationIcon = {
                BackButton(backStack)
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