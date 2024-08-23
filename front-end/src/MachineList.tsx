import {z} from "zod";
import {zodResolver} from "@hookform/resolvers/zod"
import {useForm} from "react-hook-form"

import {Button} from "@/components/ui/button"
import {
    Form,
    FormControl,
    FormDescription,
    FormField,
    FormItem,
    FormLabel,
    FormMessage,
} from "@/components/ui/form"
import {Input} from "@/components/ui/input"

import {
    ColumnDef,
    flexRender,
    getCoreRowModel,
    useReactTable,
} from "@tanstack/react-table"

import {
    Table,
    TableBody,
    TableCell,
    TableHead,
    TableHeader,
    TableRow,
} from "@/components/ui/table"
import {Card, CardContent} from "@/components/ui/card.tsx";


export function MachineList() {
    return <div className={"grow m-5"}>
        <MachineAdder/>
        <CurrentMachineList/>
    </div>
        ;
}

const machineAddFormSchema = z.object({
    name: z.string(),
    quantity: z.number().int().gt(0),
})

function MachineAdder() {
    const form = useForm<z.infer<typeof machineAddFormSchema>>({
        resolver: zodResolver(machineAddFormSchema),
        defaultValues: {
            name: "",
            quantity: 1,
        },
    })

    function onSubmit(data: z.infer<typeof machineAddFormSchema>) {
        console.log(data)
        form.reset()
    }

    function inc() {
        console.log("before", form.getValues("quantity"))
        form.setValue("quantity", form.getValues("quantity") + 1, {shouldValidate: true})
        console.log("now", form.getValues("quantity"))
    }

    function dec() {
        form.setValue("quantity", form.getValues("quantity") - 1, {shouldValidate: true})
    }

    return <Card className={"pt-6"}>
        <CardContent>

            <Form {...form}>
                <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-2">
                    <FormField
                        control={form.control}
                        name={"name"}
                        render={({field}) => (
                            <FormItem>
                                <FormLabel>Machine name</FormLabel>
                                <FormControl>
                                    <Input placeholder="machine name" autoComplete={"off"} {...field} />

                                </FormControl>
                                <FormDescription>
                                    This is the name/type of machine
                                </FormDescription>
                                <FormMessage/>
                            </FormItem>
                        )}
                    />
                    <FormField
                        control={form.control}
                        name={"quantity"}
                        render={({field}) => (
                            <FormItem>
                                <FormLabel>Machine name</FormLabel>
                                <FormControl>
                                    <span className={"flex w-full flex-row"}>
                                        <Input placeholder="machine count" type={"number"} className={"w-5/6 rounded-l-md rounded-r-none"} {...field} />
                                        <span className={"flex flex-col h-9"}>
                                            <Button className={
                                                "w-1/6 h-1/2 rounded-none rounded-tr-md text-inherit bg-transparent text-inherit bg-transparent border-r border-t"}
                                                    onClick={inc} type={"button"}>+</Button>
                                            <Button className={"w-1/6 h-1/2 rounded-none rounded-br-md text-inherit bg-transparent border-r border-b border-t"} onClick={dec} type={"button"}>-</Button>
                                        </span>

                                    </span>
                                </FormControl>
                                <FormDescription>
                                    This is the number of said machine
                                </FormDescription>
                                <FormMessage/>
                            </FormItem>
                        )}
                    />
                    <Button type="submit">Create</Button>
                </form>
            </Form>
        </CardContent>
    </Card>
}


export type MachineRow = {
    name: string,
    count: number
}

export const columns: ColumnDef<MachineRow>[] = [
    {
        accessorKey: "name",
        header: "Machine Name",
    },
    {
        accessorKey: "count",
        header: "Machine Count",
    },
]


interface MachineDataTableProps<TData, TValue> {
    columns: ColumnDef<TData, TValue>[]
    data: TData[]
}

function MachineDataTable<TData, TValue>({
                                             columns,
                                             data,
                                         }: MachineDataTableProps<TData, TValue>) {
    const table = useReactTable({
        data,
        columns,
        getCoreRowModel: getCoreRowModel(),
    })

    return (
        <div className="rounded-md border">
            <Table>
                <TableHeader className={"tableBorders"}>
                    {table.getHeaderGroups().map((headerGroup) => (
                        <TableRow key={headerGroup.id}>
                            {headerGroup.headers.map((header) => {
                                return (
                                    <TableHead key={header.id}>
                                        {header.isPlaceholder
                                            ? null
                                            : flexRender(
                                                header.column.columnDef.header,
                                                header.getContext()
                                            )}
                                    </TableHead>
                                )
                            })}
                        </TableRow>
                    ))}
                </TableHeader>
                <TableBody>
                    {table.getRowModel().rows?.length ? (
                        table.getRowModel().rows.map((row) => (
                            <TableRow
                                key={row.id}
                                data-state={row.getIsSelected() && "selected"}
                            >
                                {row.getVisibleCells().map((cell) => (
                                    <TableCell key={cell.id}>
                                        {flexRender(cell.column.columnDef.cell, cell.getContext())}
                                    </TableCell>
                                ))}
                            </TableRow>
                        ))
                    ) : (
                        <TableRow>
                            <TableCell colSpan={columns.length} className="h-24 text-center">
                                No results.
                            </TableCell>
                        </TableRow>
                    )}
                </TableBody>
            </Table>
        </div>
    )
}


function CurrentMachineList() {
    let data = [
        {"name": "electrolyzer", "count": 1},
        {"name": "crusher", "count": 1},
    ]
    return <MachineDataTable columns={columns} data={data}/>
}