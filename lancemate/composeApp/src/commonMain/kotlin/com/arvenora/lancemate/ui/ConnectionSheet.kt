package com.arvenora.lancemate.ui

import androidx.compose.runtime.Composable
import androidx.compose.foundation.layout.*
import androidx.compose.material3.*
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableIntStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import lancemate.composeapp.generated.resources.Res
import lancemate.composeapp.generated.resources.bluetooth
import lancemate.composeapp.generated.resources.usb
import org.jetbrains.compose.resources.painterResource

@OptIn(ExperimentalMaterial3ExpressiveApi::class)
@Composable
fun ConnectionSheet() {
    Column(modifier = Modifier.fillMaxWidth()) {
        val options = listOf("BLE", "USB")
        val icons = listOf(
            Res.drawable.bluetooth,
            Res.drawable.usb,
        )
        var selectedIndex by remember { mutableIntStateOf(0) }

        Text(
            modifier = Modifier.padding(8.dp),
            text = "Connection manager",
            style = MaterialTheme.typography.displaySmallEmphasized
        )
        SingleChoiceSegmentedButtonRow(modifier = Modifier.fillMaxWidth()) {
            options.forEachIndexed { index, label ->
                SegmentedButton(
                    shape = SegmentedButtonDefaults.itemShape(
                    index = index, count = options.size
                ), onClick = { selectedIndex = index }, selected = index == selectedIndex, label = {
                    Row {
                        Icon(painterResource(icons[index]), "")
                        Spacer(Modifier.size(ToggleButtonDefaults.IconSpacing))
                        Text(label)
                    }
                })
            }
        }
        Text("TODO: sheet")
    }
}