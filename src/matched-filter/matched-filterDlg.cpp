// matched-filterDlg.cpp : implementation file
//

#include "pch.h"
#include "framework.h"
#include "matched-filter.h"
#include "matched-filterDlg.h"
#include "afxdialogex.h"

#include <libmf.hpp>

#ifdef _DEBUG
#define new DEBUG_NEW
#endif

// CMatchedFilterDlg dialog

CMatchedFilterDlg::CMatchedFilterDlg(CWnd* pParent /*=nullptr*/)
    : CSimulationDialog(IDD_MATCHEDFILTER_DIALOG, pParent)
    , m_data(model::make_model_data())
{
    m_hIcon = AfxGetApp()->LoadIcon(IDR_MAINFRAME);
}

void CMatchedFilterDlg::DoDataExchange(CDataExchange* pDX)
{
    CSimulationDialog::DoDataExchange(pDX);
    DDX_Control(pDX, IDC_PLOT, m_cIPlot);
    DDX_Control(pDX, IDC_PLOT2, m_cQPlot);
    DDX_Control(pDX, IDC_PLOT3, m_cFPlot);
    DDX_Control(pDX, IDC_PLOT4, m_cResultsPlot);
    DDX_Text(pDX, IDC_EDIT1, m_data.params->n);
    DDX_Text(pDX, IDC_EDIT2, m_data.params->sampling_rate);
    DDX_Text(pDX, IDC_EDIT3, m_data.params->bitrate);
    DDX_Text(pDX, IDC_EDIT4, m_data.params->snr);
    DDX_Text(pDX, IDC_EDIT6, m_data.params->snr_from);
    DDX_Text(pDX, IDC_EDIT7, m_data.params->snr_to);
    DDX_Text(pDX, IDC_EDIT8, m_data.params->snr_count);
    DDX_Text(pDX, IDC_EDIT9, m_data.params->num_of_tests);
    DDX_Text(pDX, IDC_EDIT5, m_demoResults.err);
}

BEGIN_MESSAGE_MAP(CMatchedFilterDlg, CSimulationDialog)
    ON_WM_PAINT()
    ON_WM_QUERYDRAGICON()
    ON_BN_CLICKED(IDC_BUTTON3, &CMatchedFilterDlg::OnBnClickedButton3)
END_MESSAGE_MAP()

// CMatchedFilterDlg message handlers

BOOL CMatchedFilterDlg::OnInitDialog()
{
    CSimulationDialog::OnInitDialog();

    // Set the icon for this dialog.  The framework does this automatically
    //  when the application's main window is not a dialog
    SetIcon(m_hIcon, TRUE);            // Set big icon
    SetIcon(m_hIcon, FALSE);        // Set small icon

    m_cIPlot.plot_layer.with(
        model::make_root_drawable(m_data.i, { {
                m_data.i.plots[0].plot
        } })
    );
    m_cQPlot.plot_layer.with(
        model::make_root_drawable(m_data.q, { {
                m_data.q.plots[0].plot
        } })
    );
    m_cFPlot.plot_layer.with(
        model::make_root_drawable(m_data.f, { {
                m_data.f.plots[0].plot,
                m_data.f.plots[1].plot,
                m_data.f.plots[2].plot,
                m_data.f.plots[3].plot
        } })
    );
    m_cResultsPlot.plot_layer.with(
        model::make_root_drawable(m_data.results, { {
                m_data.results.plots[0].plot
        } })
    );

    m_cIPlot.triple_buffered = true;
    m_cQPlot.triple_buffered = true;
    m_cFPlot.triple_buffered = true;
    m_cResultsPlot.triple_buffered = true;

    // TODO: Add extra initialization here

    return TRUE;  // return TRUE  unless you set the focus to a control
}

// If you add a minimize button to your dialog, you will need the code below
//  to draw the icon.  For MFC applications using the document/view model,
//  this is automatically done for you by the framework.

void CMatchedFilterDlg::OnPaint()
{
    if (IsIconic())
    {
        CPaintDC dc(this); // device context for painting

        SendMessage(WM_ICONERASEBKGND, reinterpret_cast<WPARAM>(dc.GetSafeHdc()), 0);

        // Center icon in client rectangle
        int cxIcon = GetSystemMetrics(SM_CXICON);
        int cyIcon = GetSystemMetrics(SM_CYICON);
        CRect rect;
        GetClientRect(&rect);
        int x = (rect.Width() - cxIcon + 1) / 2;
        int y = (rect.Height() - cyIcon + 1) / 2;

        // Draw the icon
        dc.DrawIcon(x, y, m_hIcon);
    }
    else
    {
        CSimulationDialog::OnPaint();
    }
}

// The system calls this function to obtain the cursor to display while the user drags
//  the minimized window.
HCURSOR CMatchedFilterDlg::OnQueryDragIcon()
{
    return static_cast<HCURSOR>(m_hIcon);
}

void CMatchedFilterDlg::OnBnClickedButton3()
{
    UpdateData(TRUE);

    libmf::ffi::Demo d = {};
    libmf::ffi::demo(MakeParams(), &d);

    FillPlot(d.i, *m_data.i.plots[0].data);
    FillPlot(d.q, *m_data.q.plots[0].data);

    for (size_t i = 0; i < 4; ++i) {
        FillPlot(d.f[i], *m_data.f.plots[i].data);
        m_data.f.autoworld->adjust(*m_data.f.plots[i].data);
    }

    m_data.i.autoworld->setup(*m_data.i.plots[0].data);
    m_data.q.autoworld->setup(*m_data.q.plots[0].data);
    m_data.f.autoworld->flush();

    m_cIPlot.RedrawBuffer(); m_cIPlot.SwapBuffers();
    m_cIPlot.RedrawWindow();
    m_cQPlot.RedrawBuffer(); m_cQPlot.SwapBuffers();
    m_cQPlot.RedrawWindow();
    m_cFPlot.RedrawBuffer(); m_cFPlot.SwapBuffers();
    m_cFPlot.RedrawWindow();

    m_demoResults.err = d.e;

    UpdateData(FALSE);
}

libmf::ffi::Params CMatchedFilterDlg::MakeParams()
{
    libmf::ffi::Params p = {};
    p.bit_rate = m_data.params->bitrate;
    p.n = m_data.params->n;
    p.sample_rate = m_data.params->sampling_rate;
    p.snr = m_data.params->snr;
    return p;
}

void CMatchedFilterDlg::FillPlot(const libmf::ffi::Signal& s, model::points_t& pts)
{
    pts.resize((size_t)s.n);
    for (size_t i = 0; i < s.n; ++i) {
        pts[i] = { s.pts[i].x, s.pts[i].y };
    }
}
