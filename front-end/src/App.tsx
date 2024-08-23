import './App.css'

import {
    FullscreenTabs,
    FullscreenTabsContent,
    FullscreenTabsList,
    FullscreenTabsTrigger
} from "@/components/ui/fullscreenTabs.tsx"
import {MachineList} from "@/MachineList.tsx";


function App() {
    return (
        <>
            <FullscreenTabs defaultValue="configuration">
                <FullscreenTabsList>
                    <FullscreenTabsTrigger value="configuration">Configuration</FullscreenTabsTrigger>
                    <FullscreenTabsTrigger value="planning">Planning</FullscreenTabsTrigger>
                    <FullscreenTabsTrigger value="execution">Execution</FullscreenTabsTrigger>
                </FullscreenTabsList>
                <FullscreenTabsContent value="configuration">
                    <div className={"grid grid-cols-4"}>
                        <MachineList/>
                        <PartList/>
                        <RecipeList/>
                        <OrderList/>
                    </div>
                </FullscreenTabsContent>
                <FullscreenTabsContent value="planning">Change your password here.</FullscreenTabsContent>
                <FullscreenTabsContent value="execution">Change your password here.</FullscreenTabsContent>
            </FullscreenTabs>

        </>
    )
}

function PartList() {
    return <div className={"grow m-5"}>Part list</div>
}

function RecipeList() {
    return <div className={"grow m-5"}>Recipe list</div>
}

function OrderList() {
    return <div className={"grow m-5"}>Order list</div>
}

export default App
